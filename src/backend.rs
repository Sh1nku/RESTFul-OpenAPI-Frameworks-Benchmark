use crate::config::argparse::Arguments;
use crate::config::framework::FrameworkConfig;
use docker_api::opts::{
    ContainerConnectionOpts, ContainerCreateOpts, ContainerRemoveOpts, ContainerStopOpts,
    ImageBuildOpts, NetworkCreateOpts, NetworkListOptsBuilder, PublishPort, PullOpts,
};
use docker_api::{Container, Docker, Network};
use futures_util::stream::StreamExt;
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const NETWORK_NAME: &str = "restful_api_network";
const SOLR_CONTAINER_NAME: &str = "restful_api_solr";
const SOLR_IMAGE_NAME: &str = "solr";
const SOLR_IMAGE_TAG: &str = "8.11.2";

const ZOOKEEPER_CONTAINER_NAME: &str = "restful_api_zookeeper";
const ZOOKEEPER_IMAGE_NAME: &str = "zookeeper";
const ZOOKEEPER_IMAGE_TAG: &str = "3.4";

const VARNISH_CONTAINER_NAME: &str = "restful_api_varnish";
const VARNISH_IMAGE_NAME: &str = "varnish";
const VARNISH_IMAGE_TAG: &str = "6.0";

const SPEEDBUMP_IMAGE_NAME: &str = "kffl/speedbump";
const SPEEDBUMP_IMAGE_TAG: &str = "v1.1.0";

const SPEEDBUMP_FAST_CONTAINER_NAME: &str = "restful_api_speedbump_fast";
const SPEEDBUMP_FAST_COMMAND: &str = "--latency 20ms --port 8984 varnish:8983";

const SPEEDBUMP_SLOW_CONTAINER_NAME: &str = "restful_api_speedbump_slow";
const SPEEDBUMP_SLOW_COMMAND: &str = "--latency 200ms --port 8984 varnish:8983";

pub const OHA_CONTAINER_NAME: &str = "restful_api_oha";

pub async fn reset_containers(
    docker: &Docker,
    network_name: &str,
) -> Result<(), docker_api::errors::Error> {
    let networks = docker
        .networks()
        .list(&NetworkListOptsBuilder::default().build())
        .await?;
    let network = networks
        .iter()
        .find(|n| n.name == Some(network_name.to_string()));
    if network.is_none() {
        return Ok(());
    }
    let network = docker.networks().get(network_name).inspect().await?;
    match network.containers {
        None => {}
        Some(networks) => {
            for (name, _) in networks {
                reset_container(docker, name.as_str()).await;
            }
        }
    }
    debug!("Deleting network {network_name}");
    docker.networks().get(network_name).delete().await?;
    Ok(())
}

async fn reset_container(docker: &Docker, name: &str) {
    debug!("Stopping {name} container");
    match docker
        .containers()
        .get(name)
        .stop(&ContainerStopOpts::builder().build())
        .await
    {
        Ok(_) => {
            debug!("Stopped {name} container")
        }
        Err(e) => {
            debug!("{:?}", e)
        }
    };
    debug!("Deleting {name} container");
    match docker
        .containers()
        .get(name)
        .remove(&ContainerRemoveOpts::builder().volumes(true).build())
        .await
    {
        Ok(_) => {
            debug!("Deleted {name} container")
        }
        Err(e) => {
            debug!("{:?}", e)
        }
    };
}

async fn pull_image(
    docker: &Docker,
    name: &str,
    tag: &str,
) -> Result<(), docker_api::errors::Error> {
    debug!("Pulling {name} image");
    let images = docker.images();
    let mut stream = images.pull(&PullOpts::builder().image(name).tag(tag).build());
    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => {
                debug!("{:?}", output);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

async fn create_network(docker: &Docker, name: &str) -> Result<Network, docker_api::errors::Error> {
    if docker.networks().get(name).inspect().await.is_ok() {
        docker.networks().get(name).delete().await?;
    }
    docker
        .networks()
        .create(&NetworkCreateOpts::builder(name).build())
        .await
}

async fn create_container(
    docker: &Docker,
    network: &Network,
    container_name: &str,
    image_name: &str,
    image_type: ImageType,
    opts: &ContainerCreateOpts,
) -> Result<Container, docker_api::errors::Error> {
    reset_container(docker, container_name).await;
    if let ImageType::Remote(_) = image_type {
        pull_image(docker, image_name, image_type.get_tag().as_str()).await?;
    }
    debug!("Creating {container_name} container");
    let container = docker.containers().create(opts).await?;
    network
        .connect(&ContainerConnectionOpts::builder(container_name).build())
        .await?;
    container.start().await?;
    Ok(container)
}

pub async fn build_image(
    docker: &Docker,
    path: &Path,
    tag: &str,
) -> Result<(), docker_api::errors::Error> {
    debug!("Building {tag} container");
    let images = docker.images();
    let directory = path.parent().unwrap();
    let file = path.file_name().unwrap();
    let mut stream = images.build(
        &ImageBuildOpts::builder(directory)
            .dockerfile(file.to_str().unwrap())
            .tag(tag)
            .build(),
    );
    while let Some(build_result) = stream.next().await {
        match build_result {
            Ok(output) => {
                debug!("{:?}", output);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub async fn start_backend(
    docker: &Docker,
    arguments: &Arguments,
) -> Result<Network, docker_api::errors::Error> {
    let current_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    reset_containers(docker, NETWORK_NAME).await?;
    let network = create_network(docker, NETWORK_NAME).await?;
    create_container(
        docker,
        &network,
        ZOOKEEPER_CONTAINER_NAME,
        ZOOKEEPER_IMAGE_NAME,
        ImageType::Remote(ZOOKEEPER_IMAGE_TAG.to_string()),
        &ContainerCreateOpts::builder()
            .image(format!("{ZOOKEEPER_IMAGE_NAME}:{ZOOKEEPER_IMAGE_TAG}"))
            .hostname("zookeeper")
            .name(ZOOKEEPER_CONTAINER_NAME)
            .volumes([format!(
                "{current_dir}/benchmark_server/start_zk.sh:/start_zk.sh"
            )])
            .command(["/start_zk.sh"])
            .env([
                "ZOO_MY_ID=1",
                "ZOO_PORT=2181",
                "ZOO_SERVERS=server.1=zookeeper:2888:3888",
            ])
            .build(),
    )
    .await?;
    create_container(
        docker,
        &network,
        SOLR_CONTAINER_NAME,
        SOLR_IMAGE_NAME,
        ImageType::Remote(SOLR_IMAGE_TAG.to_string()),
        &ContainerCreateOpts::builder()
            .image(format!("{SOLR_IMAGE_NAME}:{SOLR_IMAGE_TAG}"))
            .hostname("solr")
            .name(SOLR_CONTAINER_NAME)
            .volumes([format!(
                "{current_dir}/benchmark_server/start_solr.sh:/start_solr.sh"
            )])
            .memory(1_000_000_000)
            .command(["/start_solr.sh"])
            .env(["ZK_HOST=zookeeper", "SOLR_JAVA_MEM=-Xms512m -Xmx512m"])
            .build(),
    )
    .await?;
    create_container(
        docker,
        &network,
        VARNISH_CONTAINER_NAME,
        VARNISH_IMAGE_NAME,
        ImageType::Remote(VARNISH_IMAGE_TAG.to_string()),
        &ContainerCreateOpts::builder()
            .image(format!("{VARNISH_IMAGE_NAME}:{VARNISH_IMAGE_TAG}"))
            .hostname("varnish")
            .name(VARNISH_CONTAINER_NAME)
            .expose(PublishPort::tcp(80), arguments.port as u32)
            .volumes([format!(
                "{current_dir}/benchmark_server/varnish.vcl:/etc/varnish/default.vcl"
            )])
            .build(),
    )
    .await?;
    if !arguments.development {
        create_container(
            docker,
            &network,
            SPEEDBUMP_FAST_CONTAINER_NAME,
            SPEEDBUMP_IMAGE_NAME,
            ImageType::Remote(SPEEDBUMP_IMAGE_TAG.to_string()),
            &ContainerCreateOpts::builder()
                .image(format!("{SPEEDBUMP_IMAGE_NAME}:{SPEEDBUMP_IMAGE_TAG}"))
                .hostname("speedbump_fast")
                .name(SPEEDBUMP_FAST_CONTAINER_NAME)
                .command(
                    SPEEDBUMP_FAST_COMMAND
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .as_slice(),
                )
                .build(),
        )
        .await?;
        create_container(
            docker,
            &network,
            SPEEDBUMP_SLOW_CONTAINER_NAME,
            SPEEDBUMP_IMAGE_NAME,
            ImageType::Remote(SPEEDBUMP_IMAGE_TAG.to_string()),
            &ContainerCreateOpts::builder()
                .image(format!("{SPEEDBUMP_IMAGE_NAME}:{SPEEDBUMP_IMAGE_TAG}"))
                .hostname("speedbump_slow")
                .name(SPEEDBUMP_SLOW_CONTAINER_NAME)
                .command(
                    SPEEDBUMP_SLOW_COMMAND
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .as_slice(),
                )
                .build(),
        )
        .await?;
    }
    Ok(network)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageType {
    Local,
    Remote(String),
}

impl ImageType {
    pub fn get_tag(&self) -> String {
        match self {
            ImageType::Local => "latest".to_string(),
            ImageType::Remote(tag) => tag.to_string(),
        }
    }
}

pub async fn start_benchmark_container(
    docker: &Docker,
    network: &Network,
    container_name: &str,
    image_type: ImageType,
    benchmark_host: &str,
) -> Result<Container, docker_api::errors::Error> {
    reset_container(docker, container_name).await;
    let image_tag = image_type.get_tag();

    create_container(
        docker,
        network,
        container_name,
        container_name,
        image_type,
        &ContainerCreateOpts::builder()
            .image(format!("{container_name}:{image_tag}"))
            .hostname(container_name)
            .name(container_name)
            .env([format!("BENCHMARK_HOST={benchmark_host}")])
            .build(),
    )
    .await
}

pub async fn start_benchmark_runner_application(
    docker: &Docker,
    network: &Network,
    image_type: ImageType,
    command: &str,
) -> Result<Container, docker_api::errors::Error> {
    reset_container(docker, OHA_CONTAINER_NAME).await;
    let image_tag = image_type.get_tag();

    create_container(
        docker,
        network,
        OHA_CONTAINER_NAME,
        OHA_CONTAINER_NAME,
        image_type,
        &ContainerCreateOpts::builder()
            .image(format!("{OHA_CONTAINER_NAME}:{image_tag}"))
            .name(OHA_CONTAINER_NAME)
            .command(command.split(' ').collect::<Vec<&str>>().as_slice())
            .build(),
    )
    .await
}
