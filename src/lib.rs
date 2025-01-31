pub mod badge;
pub mod data_source;
pub mod query;
pub mod types;
pub mod utils;

mod evm_chainlist;
mod services;

use std::{error::Error, path::PathBuf};

use actix_web::{middleware::Logger, web, App, HttpServer};
use data_source::{DataSource, SourceResponseWithMetadata};
use query::Query;

struct Executor {
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
    ) -> Result<SourceResponseWithMetadata, Box<dyn Error>> {
        let query = Query::parse_path(path)?;
        self.data_source.get_data(query).await
    }

    pub async fn get_scanner_link(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let query = Query::parse_path(path)?;
        self.data_source.get_scanner_link(query).await
    }
}

pub async fn start_server(host: &str, port: u16) {
    let ui_directory = std::env::var("UI_DIRECTORY").unwrap_or("./ui/dist".to_string());
    let default_file = get_default_file(&ui_directory);

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let executor = web::Data::new(Executor::new());

    // Start HTTP Server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(executor.clone())
            .service(services::api::health)
            .service(services::api::query)
            .service(services::badge::badge)
            .service(services::scanner::scanner)
            .service(actix_files::Files::new("/", ui_directory.clone()).index_file("index.html"))
            .default_service(actix_files::NamedFile::open(default_file.as_path()).unwrap())
    })
    .bind((host, port))
    .expect("Cannot bind to specified host and port")
    .run()
    .await
    .expect("HTTP Server crashed");
}

fn get_default_file(ui_directory: &str) -> PathBuf {
    let mut path = PathBuf::from(ui_directory);
    path.push("index.html");
    path
}
