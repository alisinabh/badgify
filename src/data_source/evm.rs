use std::error::Error;

use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
};

use crate::query::EVMQuery;

use super::SourceResponse;

const ETH_DECIMALS: u8 = 18;

pub async fn get_data(evm_query: EVMQuery) -> Result<SourceResponse, Box<dyn Error>> {
    match evm_query {
        EVMQuery::NativeBalance { chain_id, address } => {
            get_native_balance(chain_id, address).await
        }
        EVMQuery::ERC20Balance {
            chain_id,
            address,
            contract_address,
        } => get_erc20_balance(chain_id, contract_address, address),
    }
}

async fn get_native_balance(
    chain_id: U256,
    address: Address,
) -> Result<SourceResponse, Box<dyn Error>> {
    let res = ProviderBuilder::new()
        .on_http("https://eth.llamarpc.com".parse().unwrap())
        .get_balance(address)
        .await?;

    Ok(SourceResponse::NumericWithDecimals(res, ETH_DECIMALS))
}

fn get_erc20_balance(
    chain_id: U256,
    contract_address: Address,
    address: Address,
) -> Result<SourceResponse, Box<dyn Error>> {
    todo!()
}
