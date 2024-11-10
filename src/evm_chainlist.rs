use std::{
    sync::Arc,
    time::{SystemTime, SystemTimeError},
};
use tokio::sync::RwLock;

use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

use crate::types::ChainID;

const CHAIN_LIST_URL: &str = "https://chainid.network/chains.json";
const REFRESH_INTERVAL: u64 = 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvmChain {
    pub name: String,
    pub chain: String,
    pub icon: Option<String>,
    #[serde(skip_serializing)]
    pub rpc: Vec<String>,
    // #[serde(skip_serializing)]
    // pub features: Option<Vec<Feature>>,
    // #[serde(skip_serializing)]
    // pub faucets: Vec<String>,
    pub native_currency: NativeCurrency,
    // pub info_url: Option<String>,
    pub short_name: String,
    pub chain_id: U256,
    pub network_id: U256,
    pub slip44: Option<u64>,
    // pub ens: Option<Ens>,
    // #[serde(skip_serializing)]
    // pub explorers: Option<Vec<Explorer>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feature {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ens {
    registry: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Explorer {
    name: String,
    url: String,
    icon: Option<String>,
    standard: String,
}

pub struct EvmChainList {
    data: Arc<RwLock<EvmChainListData>>,
}

#[derive(Default)]
pub struct EvmChainListData {
    list: Option<Vec<EvmChain>>,
    last_fetch_at: Option<SystemTime>,
}

impl Default for EvmChainList {
    fn default() -> Self {
        Self {
            data: Arc::new(RwLock::new(EvmChainListData::default())),
        }
    }
}

impl EvmChainListData {
    pub fn get_chain(&self, chain_id: ChainID) -> Option<EvmChain> {
        if let Some(chain_list) = &self.list {
            chain_list.iter().find(|c| c.chain_id == chain_id).cloned()
        } else {
            None
        }
    }
}

impl EvmChainList {
    pub async fn fetch_evm_chain(
        &self,
        chain_id: ChainID,
    ) -> Result<Option<EvmChain>, Box<dyn std::error::Error>> {
        self.fetch_chain_list().await.unwrap();

        let data_read = self.data.read().await;

        Ok(data_read.get_chain(chain_id))
    }

    async fn fetch_chain_list(&self) -> Result<(), SystemTimeError> {
        let fetch_db = if let Some(last_fetched_at) = self.data.read().await.last_fetch_at {
            SystemTime::now().duration_since(last_fetched_at)?.as_secs() > REFRESH_INTERVAL
        } else {
            true
        };

        if fetch_db {
            let new_chain_list_data = fetch_evm_chainlist().await.unwrap();
            let mut chain_list_data = self.data.write().await;
            chain_list_data.list = Some(new_chain_list_data);
            chain_list_data.last_fetch_at = Some(SystemTime::now());
        }

        Ok(())
    }
}

async fn fetch_evm_chainlist() -> Result<Vec<EvmChain>, Box<dyn std::error::Error>> {
    Ok(reqwest::get(CHAIN_LIST_URL).await?.json().await?)
}
