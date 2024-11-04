mod bitcoin;
mod evm;

use bitcoin::BitcoinQuery;
pub use evm::{EVMQuery, EVMQueryParseError};

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
                EVMQuery::parse(path_params).map_err(|e| QueryParseError::EVMQueryParseError(e))?,
            )),
            // "bitcoin" => parse_bitcoin_query(path_params),
            invalid => Err(QueryParseError::InvalidSource(invalid.into())),
        }
    }
}
