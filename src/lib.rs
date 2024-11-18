pub mod badge;
pub mod data_source;
pub mod query;
pub mod types;
pub mod utils;

mod evm_chainlist;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};
use data_source::{DataSource, SourceResponseWithMetadata};
use query::Query;

pub struct Executor {
    data_source: DataSource,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
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
    let ui_directory = std::env::var("UI_DIRECTORY").unwrap_or("./ui/dist".to_string());

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Start HTTP Server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Executor::new()))
            .service(services::api::health)
            .service(services::api::query)
            .service(services::badge::badge)
            .service(actix_files::Files::new("/", ui_directory.clone()).index_file("index.html"))
    })
    .bind((host, port))
    .expect("Cannot bind to specified host and port")
    .run()
    .await
    .expect("HTTP Server crashed");
}
