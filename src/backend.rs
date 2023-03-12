use docker_api::{Docker};
use docker_api::opts::{ContainerConnectionOpts, ContainerCreateOpts, ContainerRemoveOpts, ContainerStopOpts, NetworkCreateOpts, PublishPort, PullOpts};
use log::{debug};
use futures_util::stream::{StreamExt};

const NETWORK_NAME: &str = "restful_api_network";
const SOLR_CONTAINER_NAME: &str = "restful_api_solr";
const SOLR_IMAGE_NAME: &str = "solr";
const SOLR_IMAGE_TAG: &str = "8.11.2";

const ZOOKEEPER_CONTAINER_NAME: &str = "restful_api_zookeeper";
const ZOOKEEPER_IMAGE_NAME: &str = "zookeeper";
const ZOOKEEPER_IMAGE_TAG: &str = "3.4";

pub async fn reset_container(docker: &Docker, name: &str) {
    debug!("Stopping previous {name} container");
    match docker.containers().get(name).stop(&ContainerStopOpts::builder().build()).await {
        Ok(_) => {debug!("Stopped previous {name} container")}
        Err(e) => {debug!("{:?}", e)}
    };
    debug!("Deleting previous {name} container");
    match docker.containers().get(name).remove(&ContainerRemoveOpts::builder().volumes(true).build()).await {
        Ok(_) => {debug!("Deleted previous {name} container")}
        Err(e) => {debug!("{:?}", e)}
    };
}

pub async fn pull_image(docker: &Docker, name: &str, tag: &str) -> Result<(), docker_api::errors::Error> {
    debug!("Pulling {name} image");
    let images = docker.images();
    let mut stream = images.pull(&PullOpts::builder().image(name).tag(tag).build());
    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => {
                debug!("{:?}", output);
            },
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

pub async fn start_backend(docker: &Docker, host_port: Option<u16>) -> Result<(), docker_api::errors::Error> {
    let current_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    reset_container(docker, ZOOKEEPER_CONTAINER_NAME).await;
    pull_image(docker, ZOOKEEPER_IMAGE_NAME, ZOOKEEPER_IMAGE_TAG).await?;
    reset_container(docker, SOLR_CONTAINER_NAME).await;
    pull_image(docker, SOLR_IMAGE_NAME, SOLR_IMAGE_TAG).await?;

    if docker.networks().get(NETWORK_NAME).inspect().await.is_ok() {
        docker.networks().get(NETWORK_NAME).delete().await?;
    }
    let network = docker.networks().create(&NetworkCreateOpts::builder(NETWORK_NAME).build()).await?;

    debug!("Creating zookeeper container");
    let zk = docker.containers().create(
        &ContainerCreateOpts::builder()
            .image(format!("{ZOOKEEPER_IMAGE_NAME}:{ZOOKEEPER_IMAGE_TAG}"))
            .hostname("zookeeper")
            .name(ZOOKEEPER_CONTAINER_NAME)
            .volumes([format!("{current_dir}/benchmark_server/start_zk.sh:/start_zk.sh")])
            .command(["/start_zk.sh"])
            .env(["ZOO_MY_ID=1", "ZOO_PORT=2181","ZOO_SERVERS=server.1=zookeeper:2888:3888"])
            .build()).await?;
    network.connect(&ContainerConnectionOpts::builder(ZOOKEEPER_CONTAINER_NAME).network_id(NETWORK_NAME).build()).await?;

    debug!("Creating solr container");
    let solr = docker.containers().create(
        &ContainerCreateOpts::builder()
            .image(format!("{SOLR_IMAGE_NAME}:{SOLR_IMAGE_TAG}"))
            .hostname("solr")
            .name(SOLR_CONTAINER_NAME)
            .volumes([format!("{current_dir}/benchmark_server/start_solr.sh:/start_solr.sh")])
            .expose(PublishPort::tcp(8983), 8983)
            .command(["/start_solr.sh"])
            .env(["ZK_HOST=zookeeper", "SOLR_JAVA_MEM=-Xms1g -Xmx1g"])
            .build()).await?;
    network.connect(&ContainerConnectionOpts::builder(SOLR_CONTAINER_NAME).network_id(NETWORK_NAME).build()).await?;

    zk.start().await?;
    solr.start().await?;

    Ok(())
}