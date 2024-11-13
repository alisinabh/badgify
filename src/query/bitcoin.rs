use std::str::Split;

use serde::Serialize;

type BitcoinAddress = String;

#[derive(Debug)]
pub enum BitcoinQuery {
    NativeBalance {
        network: BitcoinNetwork,
        address: BitcoinAddress,
    },
}

#[derive(Debug, Serialize)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Signet,
}

#[derive(Debug)]
pub enum BitcoinQueryParseError {
    BadNetwork,
    BadType,
    BadAddress,
}

impl BitcoinQuery {
    pub fn parse(mut path_params: Split<'_, &str>) -> Result<Self, BitcoinQueryParseError> {
        let network = path_params
            .next()
            .map(|network| match network.to_lowercase().as_str() {
                "mainnet" => Ok(BitcoinNetwork::Mainnet),
                "testnet" => Ok(BitcoinNetwork::Testnet),
                "signet" => Ok(BitcoinNetwork::Signet),
                _ => Err(BitcoinQueryParseError::BadNetwork),
            })
            .unwrap_or(Err(BitcoinQueryParseError::BadNetwork))?;

        match path_params
            .next()
            .ok_or(BitcoinQueryParseError::BadType)?
            .to_lowercase()
            .as_ref()
        {
            "balance" => Ok(Self::parse_native_balance(path_params, network)?),
            _ => Err(BitcoinQueryParseError::BadType),
        }
    }

    fn parse_native_balance(
        mut path_params: Split<'_, &str>,
        network: BitcoinNetwork,
    ) -> Result<Self, BitcoinQueryParseError> {
        let address = path_params
            .next()
            .map(|s| s.to_string())
            .ok_or(BitcoinQueryParseError::BadNetwork)?;

        Ok(Self::NativeBalance { network, address })
    }
}
