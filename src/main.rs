use crate::argparse::Arguments;
use crate::backend::{
    reset_containers, start_backend, start_benchmark_container, ImageType, NETWORK_NAME,
};
use crate::benchmark_config::get_benchmark_configs;
use crate::framework_config::get_framework_configs;
use crate::solr::{create_solr_client, upload_data_to_solr};
use chrono::Local;
use clap::Parser;
use docker_api::{ApiVersion, Docker};
use env_logger::Builder;
use log::{info, LevelFilter};
use std::io::Write;
use tokio::signal;

pub mod argparse;
pub mod backend;
pub mod benchmark_config;
pub mod framework_config;
pub mod solr;

#[tokio::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
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
    args.custom_validate(
        &framework_configs
            .keys()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    )
    .unwrap();
    //TODO Dynamically determine Docker version?
    let docker = Docker::unix_versioned(
        "/var/run/docker.sock",
        ApiVersion::new(1, Some(41), Some(0)),
    );
    let network = start_backend(&docker, &args).await.unwrap();
    let solr_client = create_solr_client(format!("http://127.0.0.1:{}", args.port).as_str());
    upload_data_to_solr(&solr_client).await.unwrap();

    if args.development {
        info!("In development mode. Press Ctrl-C to exit.");
        signal::ctrl_c().await.unwrap();
        info!("Received Ctrl-C. Stopping containers and exiting")
    } else {
        for config in framework_configs
            .iter()
            .filter(|(k, _v)| args.frameworks.contains(k) || args.frameworks.is_empty())
        {
            let container = start_benchmark_container(
                &docker,
                &network,
                config.0.as_str(),
                config.1,
                ImageType::Local,
            )
            .await
            .unwrap();
        }
    }
    reset_containers(&docker, NETWORK_NAME).await.unwrap();
}
