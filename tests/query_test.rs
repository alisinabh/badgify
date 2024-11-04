use marketh_rs::query::{EVMQuery, Query};
use num::BigUint;
use num::FromPrimitive;

#[test]
fn test_parsing_valid_evm_balance_query() {
    let res = Query::parse_path("evm/1/balance/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

    assert!(matches!(
        res,
        Ok(Query::EVM(EVMQuery::NativeBalance {
            chain_id,
            address,
        })) if chain_id == BigUint::from_u8(1).unwrap() &&
              address == "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string()
    ));
}

#[test]
fn test_parsing_valid_evm_erc20_query() {
    let res = Query::parse_path("evm/8/erc20_balance/0xdac17f958d2ee523a2206206994597c13d831ec7/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

    assert!(matches!(
        res,
        Ok(Query::EVM(EVMQuery::ERC20Balance {
            chain_id,
            contract_address,
            address,
        })) if chain_id == BigUint::from_u8(8).unwrap() &&
               contract_address == "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string() &&
              address == "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string()
    ));
}
