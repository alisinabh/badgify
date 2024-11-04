mod evm;

use alloy::primitives::U256;

use crate::query::Query;
use std::error::Error;

#[derive(Debug)]
pub enum SourceResponse {
    NumericWithDecimals(U256, u8),
    AlphaNumeric(String),
}

pub async fn get_data(query: Query) -> Result<SourceResponse, Box<dyn Error>> {
    match query {
        Query::EVM(evm_query) => evm::get_data(evm_query).await,
        Query::Bitcoin(_bitcoin_query) => todo!(),
    }
}
