use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

const CHAIN_LIST_URL: &str = "https://chainid.network/chains.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvmChain {
    pub name: String,
    pub chain: String,
    pub icon: Option<String>,
    pub rpc: Vec<String>,
    pub features: Option<Vec<Feature>>, // Optional as it might be empty or absent
    pub faucets: Vec<String>,
    pub native_currency: NativeCurrency,
    pub info_url: Option<String>,
    pub short_name: String,
    pub chain_id: U256,
    pub network_id: U256,
    pub slip44: Option<u64>,
    pub ens: Option<Ens>,
    pub explorers: Option<Vec<Explorer>>, // Optional as it might be empty or absent
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Feature {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NativeCurrency {
    name: String,
    symbol: String,
    decimals: u8,
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

async fn fetch_evm_chainlist() -> Result<Vec<EvmChain>, Box<dyn std::error::Error>> {
    Ok(reqwest::get(CHAIN_LIST_URL).await?.json().await?)
}

pub async fn fetch_evm_chain(
    chain_id: U256,
) -> Result<Option<EvmChain>, Box<dyn std::error::Error>> {
    let chain_list = fetch_evm_chainlist().await?;

    Ok(chain_list.iter().find(|c| c.chain_id == chain_id).cloned())
}
