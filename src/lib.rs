pub mod query;
pub mod types;
pub mod utils;

mod data_source;
mod evm_chainlist;
mod services;

use actix_web::{web, App, HttpServer};
use data_source::{DataSource, SourceResponseWithMetadata};
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
    ) -> Result<SourceResponseWithMetadata, Box<dyn std::error::Error>> {
        let query = Query::parse_path(path)?;
        self.data_source.get_data(query).await
    }
}

pub async fn start_server(host: &str, port: u16) {
    // Start HTTP Server
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Executor::new()))
            .service(services::api::health)
            .service(services::api::query)
    })
    .bind((host, port))
    .expect("Cannot bind to specified host and port")
    .run()
    .await
    .expect("HTTP Server crashed");
}
