mod data_source;
mod evm_chainlist;
pub mod query;
pub mod types;
pub mod utils;

use data_source::{DataSource, SourceResponse};
use query::Query;

pub struct Executor {
    data_source: DataSource,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            data_source: DataSource::default(),
        }
    }

    pub async fn query_data(
        &self,
        path: &str,
    ) -> Result<SourceResponse, Box<dyn std::error::Error>> {
        let query = Query::parse_path(path)?;
        self.data_source.get_data(query).await
    }
}
