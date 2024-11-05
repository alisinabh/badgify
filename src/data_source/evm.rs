use std::{collections::HashMap, error::Error, future::Future, sync::Arc, sync::RwLock};

use alloy::{
    primitives::{Address, Bytes},
    providers::{Provider, ProviderBuilder},
    rpc::{
        client::{ClientBuilder, ReqwestClient, Waiter},
        types::{TransactionInput, TransactionRequest},
    },
    sol,
    sol_types::SolCall,
};

use crate::{evm_chainlist::EVMChainList, query::EVMQuery, types::ChainID};

use super::SourceResponse;

const ETH_DECIMALS: u8 = 18;

sol! {
    #[sol(rpc)]
    contract ERC20 {
        #[derive(Debug)]
        function balanceOf(address owner) public view returns (uint256 balance);

        #[derive(Debug)]
        function decimals() public view returns (uint8 decimals);
    }
}

pub struct EVMDataSource {
    last_known_good_rpc_urls: Arc<RwLock<HashMap<ChainID, String>>>,
    chain_list: EVMChainList,
}

impl Default for EVMDataSource {
    fn default() -> Self {
        Self {
            last_known_good_rpc_urls: Arc::new(HashMap::new().into()),
            chain_list: EVMChainList::default(),
        }
    }
}

impl EVMDataSource {
    pub async fn get_data(&self, evm_query: EVMQuery) -> Result<SourceResponse, Box<dyn Error>> {
        match evm_query {
            EVMQuery::NativeBalance { chain_id, address } => {
                self.get_native_balance(chain_id, address).await
            }
            EVMQuery::ERC20Balance {
                chain_id,
                address,
                contract_address,
            } => {
                self.get_erc20_balance(chain_id, contract_address, address)
                    .await
            }
        }
    }

    async fn get_native_balance(
        &self,
        chain_id: ChainID,
        address: Address,
    ) -> Result<SourceResponse, Box<dyn Error>> {
        self.try_with_rpc_urls_provider(chain_id, move |provider| async move {
            match provider.get_balance(address).await {
                Ok(res) => Ok(SourceResponse::Decimal {
                    value: res,
                    decimals: ETH_DECIMALS,
                }),
                Err(err) => Err(Box::new(err) as Box<dyn Error>),
            }
        })
        .await
    }

    async fn get_erc20_balance(
        &self,
        chain_id: ChainID,
        contract_address: Address,
        address: Address,
    ) -> Result<SourceResponse, Box<dyn Error>> {
        self.try_with_rpc_urls_client(chain_id, move |client| async move {
            let mut batch = client.new_batch();

            let balance_call = ERC20::balanceOfCall::new((address,));
            let decimals_call = ERC20::decimalsCall::new(());

            let balance_fut: Waiter<Bytes> = batch
                .add_call(
                    "eth_call",
                    &[TransactionRequest::default()
                        .to(contract_address)
                        .input(TransactionInput::from(balance_call.abi_encode()))],
                )
                .unwrap();

            let decimals_fut: Waiter<Bytes> = batch
                .add_call(
                    "eth_call",
                    &[TransactionRequest::default()
                        .to(contract_address)
                        .input(TransactionInput::from(decimals_call.abi_encode()))],
                )
                .unwrap();

            if let Ok(_) = batch.send().await {
                match (balance_fut.await, decimals_fut.await) {
                    (Ok(balance), Ok(decimals)) => {
                        let balance =
                            ERC20::balanceOfCall::abi_decode_returns(&balance, true)?.balance;
                        let decimals =
                            ERC20::decimalsCall::abi_decode_returns(&decimals, true)?.decimals;
                        Ok(SourceResponse::Decimal {
                            value: balance,
                            decimals,
                        })
                    }
                    err => Err(format!("Failed to get balance or decimals {err:?}").into()),
                }
            } else {
                Err("RPC batch request failed".into())
            }
        })
        .await
    }

    async fn try_with_rpc_urls_client<F, Fut, T>(
        &self,
        chain_id: ChainID,
        predicate: F,
    ) -> Result<T, Box<dyn Error>>
    where
        F: Fn(ReqwestClient) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T, Box<dyn Error>>> + Send + 'static,
        T: Send + 'static,
    {
        self.try_with_rpc_urls(chain_id, move |rpc_url| {
            let predicate = predicate.clone();
            async move {
                let client = ClientBuilder::default().http(rpc_url.parse()?);
                predicate(client).await
            }
        })
        .await
    }

    async fn try_with_rpc_urls_provider<F, Fut, T>(
        &self,
        chain_id: ChainID,
        predicate: F,
    ) -> Result<T, Box<dyn Error>>
    where
        F: Fn(Box<dyn Provider>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T, Box<dyn Error>>> + Send + 'static,
        T: Send + 'static,
    {
        self.try_with_rpc_urls(chain_id, move |rpc_url| {
            let predicate = predicate.clone();
            async move {
                let provider = ProviderBuilder::default().on_http(rpc_url.parse()?).boxed();
                predicate(Box::new(provider)).await
            }
        })
        .await
    }

    async fn try_with_rpc_urls<'a, F, Fut, T>(
        &self,
        chain_id: ChainID,
        predicate: F,
    ) -> Result<T, Box<dyn Error>>
    where
        F: Fn(String) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T, Box<dyn Error>>> + Send + 'static,
        T: Send + 'static,
    {
        let chain = self
            .chain_list
            .fetch_evm_chain(chain_id)
            .await?
            .ok_or("Chain not found!")?;

        let mut rpc_urls = chain.rpc;

        if let Some(rpc_url) = self.get_good_rpc_url(chain_id) {
            rpc_urls.insert(0, rpc_url);
        }

        let rpc_urls_iter = rpc_urls
            .iter()
            .filter(|x| !x.contains("API_KEY") && x.starts_with("http"));

        for rpc_url in rpc_urls_iter {
            let predicate = predicate.clone();

            match predicate(rpc_url.to_string()).await {
                Ok(result) => {
                    self.set_good_rpc_url(chain_id, rpc_url);
                    return Ok(result);
                }

                Err(err) => println!("Error with provider at {}: {:?}", rpc_url, err),
            }
        }

        Err("No active RPC URLs for chain".into())
    }

    fn set_good_rpc_url(&self, chain_id: ChainID, rpc_url: &str) {
        let should_update = match self.last_known_good_rpc_urls.read().unwrap().get(&chain_id) {
            Some(existing) => existing != rpc_url,
            None => true,
        };

        if should_update {
            self.last_known_good_rpc_urls
                .write()
                .unwrap()
                .insert(chain_id, rpc_url.to_string());
        }
    }

    fn get_good_rpc_url(&self, chain_id: ChainID) -> Option<String> {
        let res = self
            .last_known_good_rpc_urls
            .read()
            .unwrap()
            .get(&chain_id)
            .cloned();
        res
    }
}
