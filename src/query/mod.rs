mod bitcoin;
pub mod evm;

pub use evm::{EVMQuery, EVMQueryParseError};

use bitcoin::BitcoinQuery;

#[derive(Debug)]
pub enum Query {
    EVM(EVMQuery),
    Bitcoin(BitcoinQuery),
}

#[derive(Debug)]
pub enum QueryParseError {
    SourceNotFoundError,
    InvalidSource(String),
    EVMQueryParseError(EVMQueryParseError),
}

impl Query {
    pub fn parse_path(path: &str) -> Result<Self, QueryParseError> {
        let mut path_params = path.split("/");
        let source_type = path_params
            .next()
            .ok_or(QueryParseError::SourceNotFoundError)?;

        match source_type.to_lowercase().as_str() {
            "evm" => Ok(Query::EVM(
                EVMQuery::parse(path_params).map_err(QueryParseError::EVMQueryParseError)?,
            )),
            // "bitcoin" => parse_bitcoin_query(path_params),
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
