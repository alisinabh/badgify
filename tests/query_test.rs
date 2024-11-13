use alloy::primitives::{address, U256};
use badgify_rs::query::{EvmQuery, Query};

#[test]
fn test_parsing_valid_evm_balance_query() {
    let res = Query::parse_path("evm/1/balance/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

    assert!(matches!(
        res,
        Ok(Query::Evm(EvmQuery::NativeBalance {
            chain_id,
            address,
        })) if chain_id == U256::from(1_u8) &&
              address == address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
    ));
}

#[test]
fn test_parsing_valid_evm_erc20_query() {
    let res = Query::parse_path("evm/8/erc20_balance/0xdac17f958d2ee523a2206206994597c13d831ec7/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

    assert!(matches!(
        res,
        Ok(Query::Evm(EvmQuery::ERC20Balance {
            chain_id,
            contract_address,
            address,
        })) if chain_id == U256::from(8_u8) &&
               contract_address == address!("dac17f958d2ee523a2206206994597c13d831ec7") &&
              address == address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
    ));
}
