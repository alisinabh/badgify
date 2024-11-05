use crate::types::{ChainID, EVMAddress};
use crate::utils::{EVMAddressInteratorExt, Uint256IteratorExt};
use std::str::Split;

#[derive(Debug)]
pub enum EVMQuery {
    NativeBalance {
        chain_id: ChainID,
        address: EVMAddress,
    },
    ERC20Balance {
        chain_id: ChainID,
        address: EVMAddress,
        contract_address: EVMAddress,
    },
}

#[derive(Debug)]
pub enum EVMQueryParseError {
    BadChainID,
    BadType,
    BadAddress,
}

impl EVMQuery {
    pub fn parse(mut path_params: Split<'_, &str>) -> Result<EVMQuery, EVMQueryParseError> {
        let chain_id = path_params
            .next_uint256()
            .map_err(|_| EVMQueryParseError::BadChainID)?;

        match path_params
            .next()
            .ok_or(EVMQueryParseError::BadType)?
            .to_lowercase()
            .as_ref()
        {
            "balance" => Ok(Self::parse_native_balance(path_params, chain_id)?),
            "erc20_balance" => Ok(Self::parse_erc20_balance(path_params, chain_id)?),
            _ => Err(EVMQueryParseError::BadType),
        }
    }

    fn parse_native_balance(
        mut path_params: Split<'_, &str>,
        chain_id: ChainID,
    ) -> Result<EVMQuery, EVMQueryParseError> {
        let address = path_params
            .next_evm_address()
            .map_err(|_| EVMQueryParseError::BadAddress)?;

        Ok(EVMQuery::NativeBalance { chain_id, address })
    }

    fn parse_erc20_balance(
        mut path_params: Split<'_, &str>,
        chain_id: ChainID,
    ) -> Result<EVMQuery, EVMQueryParseError> {
        let contract_address = path_params
            .next_evm_address()
            .map_err(|_| EVMQueryParseError::BadAddress)?;

        let address = path_params
            .next_evm_address()
            .map_err(|_| EVMQueryParseError::BadAddress)?;

        Ok(EVMQuery::ERC20Balance {
            chain_id,
            address,
            contract_address,
        })
    }
}
