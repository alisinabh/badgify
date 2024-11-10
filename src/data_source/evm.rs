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

use crate::{
    evm_chainlist::{EvmChain, EvmChainList},
    query::EvmQuery,
    types::ChainID,
};

use super::{
    evm_metadata::{EvmMetadata, EvmSource},
    SourceMetadata, SourceResponse, SourceResponseWithMetadata,
};

const ETH_DECIMALS: u8 = 18;

sol! {
    #[sol(rpc)]
    contract ERC20 {
        #[derive(Debug)]
        function balanceOf(address owner) public view returns (uint256 balance);

        #[derive(Debug)]
        function decimals() public view returns (uint8 decimals);

        #[derive(Debug)]
        function symbol() public view returns (string symbol);
    }
}

pub struct EvmDataSource {
    last_known_good_rpc_urls: Arc<RwLock<HashMap<ChainID, String>>>,
    chain_list: EvmChainList,
}

impl Default for EvmDataSource {
    fn default() -> Self {
        Self {
            last_known_good_rpc_urls: Arc::new(HashMap::new().into()),
            chain_list: EvmChainList::default(),
        }
    }
}

impl EvmDataSource {
    pub async fn get_data(
        &self,
        evm_query: EvmQuery,
    ) -> Result<SourceResponseWithMetadata, Box<dyn Error>> {
        match evm_query {
            EvmQuery::NativeBalance { chain_id, address } => {
                self.get_native_balance(chain_id, address).await
            }
            EvmQuery::ERC20Balance {
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
    ) -> Result<SourceResponseWithMetadata, Box<dyn Error>> {
        self.try_with_rpc_urls_provider(chain_id, move |chain, provider| async move {
            match provider.get_balance(address).await {
                Ok(res) => {
                    let result = SourceResponse::Decimal {
                        value: res,
                        decimals: ETH_DECIMALS,
                    };

                    let symbol = chain.native_currency.symbol.clone();

                    let metadata = SourceMetadata::Evm(EvmMetadata::new(
                        chain,
                        EvmSource::NativeCurrency { symbol },
                    ));

                    Ok(SourceResponseWithMetadata::new(result, metadata))
                }
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
    ) -> Result<SourceResponseWithMetadata, Box<dyn Error>> {
        self.try_with_rpc_urls_client(chain_id, move |chain, client| async move {
            let mut batch = client.new_batch();

            let balance_call = ERC20::balanceOfCall::new((address,));
            let decimals_call = ERC20::decimalsCall::new(());
            let symbol_call = ERC20::symbolCall::new(());

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

            let symbol_fut: Waiter<Bytes> = batch
                .add_call(
                    "eth_call",
                    &[TransactionRequest::default()
                        .to(contract_address)
                        .input(TransactionInput::from(symbol_call.abi_encode()))],
                )
                .unwrap();

            if batch.send().await.is_ok() {
                match (balance_fut.await, decimals_fut.await, symbol_fut.await) {
                    (Ok(balance), Ok(decimals), Ok(symbol)) => {
                        let balance =
                            ERC20::balanceOfCall::abi_decode_returns(&balance, true)?.balance;
                        let decimals =
                            ERC20::decimalsCall::abi_decode_returns(&decimals, true)?.decimals;
                        let symbol = ERC20::symbolCall::abi_decode_returns(&symbol, true)?.symbol;

                        let result = SourceResponse::Decimal {
                            value: balance,
                            decimals,
                        };

                        let metadata = SourceMetadata::Evm(EvmMetadata::new(
                            chain,
                            EvmSource::ERC20 {
                                symbol,
                                contract_address,
                            },
                        ));

                        Ok(SourceResponseWithMetadata::new(result, metadata))
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
        F: Fn(Arc<EvmChain>, ReqwestClient) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T, Box<dyn Error>>> + Send + 'static,
        T: Send + 'static,
    {
        self.try_with_rpc_urls(chain_id, move |chain, rpc_url| {
            let predicate = predicate.clone();
            async move {
                let client = ClientBuilder::default().http(rpc_url.parse()?);
                predicate(chain.clone(), client).await
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
        F: Fn(Arc<EvmChain>, Box<dyn Provider>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T, Box<dyn Error>>> + Send + 'static,
        T: Send + 'static,
    {
        self.try_with_rpc_urls(chain_id, move |chain, rpc_url| {
            let predicate = predicate.clone();
            async move {
                let provider = ProviderBuilder::default().on_http(rpc_url.parse()?).boxed();
                predicate(chain.clone(), Box::new(provider)).await
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
        F: Fn(Arc<EvmChain>, String) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T, Box<dyn Error>>> + Send + 'static,
        T: Send + 'static,
    {
        let chain = Arc::new(
            self.chain_list
                .fetch_evm_chain(chain_id)
                .await?
                .ok_or("Chain not found!")?,
        );

        let mut rpc_urls = chain.rpc.clone();

        if let Some(rpc_url) = self.get_good_rpc_url(chain_id) {
            rpc_urls.insert(0, rpc_url);
        }

        let rpc_urls_iter = rpc_urls
            .iter()
            .filter(|x| !x.contains("API_KEY") && x.starts_with("http"));

        for rpc_url in rpc_urls_iter {
            let predicate = predicate.clone();

            match predicate(chain.clone(), rpc_url.to_string()).await {
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
