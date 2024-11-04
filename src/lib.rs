mod data_source;
mod query;
mod utils;

use data_source::SourceResponse;
use query::Query;

pub async fn query_data(path: &str) -> Result<SourceResponse, Box<dyn std::error::Error>> {
    let query = Query::parse_path(path)?;
    data_source::get_data(query).await
}
