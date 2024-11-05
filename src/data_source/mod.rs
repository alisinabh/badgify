mod evm;

use alloy::primitives::U256;

use crate::query::Query;
use std::error::Error;

#[derive(Debug)]
pub enum SourceResponse {
    Decimal(U256, u8),
    AlphaNumeric(String),
}

pub struct DataSource {
    evm_data_source: evm::EVMDataSource,
}

impl Default for DataSource {
    fn default() -> Self {
        Self {
            evm_data_source: evm::EVMDataSource::default(),
        }
    }
}

impl DataSource {
    pub async fn get_data(&self, query: Query) -> Result<SourceResponse, Box<dyn Error>> {
        match query {
            Query::EVM(evm_query) => self.evm_data_source.get_data(evm_query).await,
            Query::Bitcoin(_bitcoin_query) => todo!(),
        }
    }
}
