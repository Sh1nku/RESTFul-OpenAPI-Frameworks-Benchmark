use crate::argparse::{Arguments};
use docker_api::{ApiVersion, Docker};
use env_logger::Builder;
use chrono::Local;
use log::{LevelFilter};
use std::io::Write;
use crate::framework_config::get_framework_configs;
use clap::Parser;
use crate::backend::{ImageType, NETWORK_NAME, reset_containers, start_backend, start_benchmark_container};
use crate::benchmark_config::get_benchmark_configs;
use crate::solr::{create_solr_client, upload_data_to_solr};

pub mod argparse;
pub mod framework_config;
pub mod backend;
pub mod solr;
mod benchmark_config;

#[tokio::main]
async fn main() {

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let args = Arguments::parse();
    let framework_configs = get_framework_configs().unwrap();
    let benchmark_configs = get_benchmark_configs().unwrap();
    //TODO Dynamically determine Docker version?
    let docker = Docker::unix_versioned("/var/run/docker.sock", ApiVersion::new(1, Some(41), Some(0)));
    let network = start_backend(&docker).await.unwrap();
    let solr_client = create_solr_client("http://127.0.0.1:8983");
    upload_data_to_solr(&solr_client).await.unwrap();

    for config in framework_configs {
        let container = start_benchmark_container(&docker, &network, config.0.as_str(), &config.1, ImageType::Local).await.unwrap();
    }
    reset_containers(&docker, NETWORK_NAME).await.unwrap();
    
}
