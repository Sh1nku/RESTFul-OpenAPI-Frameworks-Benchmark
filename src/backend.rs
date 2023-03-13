use docker_api::{Container, Docker, Network};
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

pub async fn reset_containers(docker: &Docker, network_name: &str) -> Result<(), docker_api::errors::Error> {
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

async fn pull_image(docker: &Docker, name: &str, tag: &str) -> Result<(), docker_api::errors::Error> {
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

async fn create_network(docker: &Docker, name: &str) -> Result<Network, docker_api::errors::Error> {
    if docker.networks().get(name).inspect().await.is_ok() {
        docker.networks().get(name).delete().await?;
    }
    docker.networks().create(&NetworkCreateOpts::builder(name).build()).await
}

async fn create_container(docker: &Docker, network: &Network, container_name: &str, image_name: &str, image_tag: &str, opts: &ContainerCreateOpts) -> Result<Container, docker_api::errors::Error> {
    reset_container(docker, container_name).await;
    pull_image(docker, image_name, image_tag).await?;
    debug!("Creating {container_name} container");
    let container = docker.containers().create(opts).await?;
    network.connect(&ContainerConnectionOpts::builder(container_name).build()).await?;
    container.start().await?;
    Ok(container)
}


pub async fn start_backend(docker: &Docker, host_port: u16) -> Result<(), docker_api::errors::Error> {
    let current_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    reset_containers(docker, NETWORK_NAME).await?;
    let network = create_network(docker, NETWORK_NAME).await?;
    create_container(docker, &network, ZOOKEEPER_CONTAINER_NAME, ZOOKEEPER_IMAGE_NAME, ZOOKEEPER_IMAGE_TAG,
                     &ContainerCreateOpts::builder()
                         .image(format!("{ZOOKEEPER_IMAGE_NAME}:{ZOOKEEPER_IMAGE_TAG}"))
                         .hostname("zookeeper")
                         .name(ZOOKEEPER_CONTAINER_NAME)
                         .volumes([format!("{current_dir}/benchmark_server/start_zk.sh:/start_zk.sh")])
                         .command(["/start_zk.sh"])
                         .env(["ZOO_MY_ID=1", "ZOO_PORT=2181","ZOO_SERVERS=server.1=zookeeper:2888:3888"])
                         .build()).await?;
    create_container(docker, &network, SOLR_CONTAINER_NAME, SOLR_IMAGE_NAME, SOLR_IMAGE_TAG,
                     &ContainerCreateOpts::builder()
                         .image(format!("{SOLR_IMAGE_NAME}:{SOLR_IMAGE_TAG}"))
                         .hostname("solr")
                         .name(SOLR_CONTAINER_NAME)
                         .volumes([format!("{current_dir}/benchmark_server/start_solr.sh:/start_solr.sh")])
                         .expose(PublishPort::tcp(8983), host_port as u32)
                         .command(["/start_solr.sh"])
                         .env(["ZK_HOST=zookeeper", "SOLR_JAVA_MEM=-Xms1g -Xmx1g"])
                         .build()
    ).await?;
    Ok(())
}