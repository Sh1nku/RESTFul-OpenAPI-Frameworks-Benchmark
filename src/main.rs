use crate::argparse::{Arguments};
use docker_api::{ApiVersion, Docker};
use env_logger::Builder;
use chrono::Local;
use log::{LevelFilter};
use std::io::Write;
use crate::framework_config::get_framework_configs;
use clap::Parser;
use crate::backend::start_backend;
use crate::solr::upload_data_to_solr;

pub mod argparse;
pub mod framework_config;
pub mod backend;
pub mod solr;

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
    let configs = get_framework_configs().unwrap();
    //TODO Dynamically determine Docker version?
    let docker = Docker::unix_versioned("/var/run/docker.sock", ApiVersion::new(1, Some(41), Some(0)));
    start_backend(&docker, args.port).await.unwrap();
    upload_data_to_solr("http://127.0.0.1:8983").await.unwrap();
    
}
