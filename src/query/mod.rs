pub mod bitcoin;
pub mod evm;

pub use evm::{EvmQuery, EvmQueryParseError};

use bitcoin::{BitcoinQuery, BitcoinQueryParseError};

#[derive(Debug)]
pub enum Query {
    Evm(EvmQuery),
    Bitcoin(BitcoinQuery),
}

#[derive(Debug)]
pub enum QueryParseError {
    SourceNotFoundError,
    InvalidSource(String),
    EvmQueryParseError(EvmQueryParseError),
    BitcoinQueryParseError(BitcoinQueryParseError),
}

impl Query {
    pub fn parse_path(path: &str) -> Result<Self, QueryParseError> {
        let mut path_params = path.split("/");
        let source_type = path_params
            .next()
            .ok_or(QueryParseError::SourceNotFoundError)?;

        match source_type.to_lowercase().as_str() {
            "evm" => Ok(Query::Evm(
                EvmQuery::parse(path_params).map_err(QueryParseError::EvmQueryParseError)?,
            )),
            "btc" => Ok(Query::Bitcoin(
                BitcoinQuery::parse(path_params)
                    .map_err(QueryParseError::BitcoinQueryParseError)?,
            )),
            invalid => Err(QueryParseError::InvalidSource(invalid.into())),
        }
    }
}

impl std::fmt::Display for QueryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QueryParseError")
    }
}

impl std::error::Error for QueryParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
