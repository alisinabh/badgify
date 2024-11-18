use alloy::primitives::U256;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::query::bitcoin::{BitcoinNetwork, BitcoinQuery};

use super::{
    bitcoing_metadata::BitcoinMetadata, SourceMetadata, SourceResponse, SourceResponseWithMetadata,
};

#[derive(Default)]
pub struct BitcoinDataSource;

impl BitcoinDataSource {
    pub async fn get_data(
        &self,
        query: BitcoinQuery,
    ) -> Result<SourceResponseWithMetadata, Box<dyn Error>> {
        match query {
            BitcoinQuery::NativeBalance { network, address } => {
                self.get_native_balance(network, address).await
            }
        }
    }

    pub async fn get_scanner_link(&self, query: BitcoinQuery) -> Result<String, Box<dyn Error>> {
        match query {
            BitcoinQuery::NativeBalance { network, address } => {
                Ok(get_scanner_link(&network, &address))
            }
        }
    }

    async fn get_native_balance(
        &self,
        network: BitcoinNetwork,
        address: String,
    ) -> Result<SourceResponseWithMetadata, Box<dyn Error>> {
        let address_info = get_address_info(&network, &address).await?;

        let result = SourceResponse::Decimal {
            value: U256::from(
                address_info.chain_stats.funded_txo_sum - address_info.chain_stats.spent_txo_sum,
            ),
            decimals: 8u8,
        };

        let metadata = SourceMetadata::Bitcoin(BitcoinMetadata::new(network));

        Ok(SourceResponseWithMetadata::new(result, metadata))
    }
}

#[derive(Serialize, Deserialize)]
struct MempoolIoResponse {
    address: String,
    chain_stats: MempoolIoChainStats,
    mempool_stats: MempoolIoMempoolStats,
}

#[derive(Serialize, Deserialize)]
struct MempoolIoChainStats {
    funded_txo_count: u32,
    funded_txo_sum: u64,
    spent_txo_count: u32,
    spent_txo_sum: u64,
    tx_count: u32,
}

#[derive(Serialize, Deserialize)]
struct MempoolIoMempoolStats {
    funded_txo_count: u32,
    funded_txo_sum: u64,
    spent_txo_count: u32,
    spent_txo_sum: u64,
    tx_count: u32,
}

async fn get_address_info(
    network: &BitcoinNetwork,
    address: &str,
) -> Result<MempoolIoResponse, Box<dyn Error>> {
    let mut req_url = "https://mempool.space".to_string();

    match network {
        BitcoinNetwork::Mainnet => (),
        BitcoinNetwork::Testnet => req_url.push_str("/testnet"),
        BitcoinNetwork::Signet => req_url.push_str("/signet"),
    }

    req_url.push_str("/api/address/");
    req_url.push_str(address);

    Ok(reqwest::get(req_url).await?.json().await?)
}

fn get_scanner_link(network: &BitcoinNetwork, address: &str) -> String {
    let mut req_url = "https://mempool.space".to_string();

    match network {
        BitcoinNetwork::Mainnet => (),
        BitcoinNetwork::Testnet => req_url.push_str("/testnet"),
        BitcoinNetwork::Signet => req_url.push_str("/signet"),
    }

    req_url.push_str("/address/");
    req_url.push_str(address);

    req_url
}
