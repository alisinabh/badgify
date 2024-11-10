use crate::types::{ChainID, EvmAddress};
use crate::utils::{EvmAddressInteratorExt, Uint256IteratorExt};
use std::str::Split;

#[derive(Debug)]
pub enum EvmQuery {
    NativeBalance {
        chain_id: ChainID,
        address: EvmAddress,
    },
    ERC20Balance {
        chain_id: ChainID,
        address: EvmAddress,
        contract_address: EvmAddress,
    },
}

#[derive(Debug)]
pub enum EvmQueryParseError {
    BadChainID,
    BadType,
    BadAddress,
}

impl EvmQuery {
    pub fn parse(mut path_params: Split<'_, &str>) -> Result<EvmQuery, EvmQueryParseError> {
        let chain_id = path_params
            .next_uint256()
            .map_err(|_| EvmQueryParseError::BadChainID)?;

        match path_params
            .next()
            .ok_or(EvmQueryParseError::BadType)?
            .to_lowercase()
            .as_ref()
        {
            "balance" => Ok(Self::parse_native_balance(path_params, chain_id)?),
            "erc20_balance" => Ok(Self::parse_erc20_balance(path_params, chain_id)?),
            _ => Err(EvmQueryParseError::BadType),
        }
    }

    fn parse_native_balance(
        mut path_params: Split<'_, &str>,
        chain_id: ChainID,
    ) -> Result<EvmQuery, EvmQueryParseError> {
        let address = path_params
            .next_evm_address()
            .map_err(|_| EvmQueryParseError::BadAddress)?;

        Ok(EvmQuery::NativeBalance { chain_id, address })
    }

    fn parse_erc20_balance(
        mut path_params: Split<'_, &str>,
        chain_id: ChainID,
    ) -> Result<EvmQuery, EvmQueryParseError> {
        let contract_address = path_params
            .next_evm_address()
            .map_err(|_| EvmQueryParseError::BadAddress)?;

        let address = path_params
            .next_evm_address()
            .map_err(|_| EvmQueryParseError::BadAddress)?;

        Ok(EvmQuery::ERC20Balance {
            chain_id,
            address,
            contract_address,
        })
    }
}
