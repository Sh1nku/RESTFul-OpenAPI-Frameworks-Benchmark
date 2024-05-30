use std::error::Error;
use std::path::Path;
use chrono::Duration;
use log::debug;
use serde::{Serialize, Deserialize};
use tokio::time::sleep;
use rand::Rng;
use solrstice::clients::async_cloud_client::AsyncSolrCloudClient;
use solrstice::hosts::solr_server_host::SolrSingleServerHost;
use solrstice::models::context::SolrServerContextBuilder;
use solrstice::queries::index::UpdateQuery;

const CONFIG_NAME: &str = "test_data";
const COLLECTION_NAME: &str = "test_data";

pub fn create_solr_client(host: &str) -> AsyncSolrCloudClient {
    AsyncSolrCloudClient::new(SolrServerContextBuilder::new(SolrSingleServerHost::new(host)))
}

pub async fn upload_data_to_solr(client: &AsyncSolrCloudClient) -> Result<(), Box<dyn Error>> {
    wait_for_solr(&client, Duration::seconds(30)).await?;
    client.upload_config(CONFIG_NAME, Path::new("benchmark_server/solrconfig")).await?;
    client.create_collection(COLLECTION_NAME, CONFIG_NAME, 1, 1).await?;
    upload_test_data(&client, COLLECTION_NAME).await?;
    Ok(())
}

async fn wait_for_solr(client: &AsyncSolrCloudClient, timeout: Duration) -> Result<(), Box<dyn Error>> {
    let until = chrono::Local::now() + timeout;
    debug!("Waiting {timeout} for solr");
    loop {
        if until < chrono::Local::now() {
            return Err("Could not reach solr within timeout".into());
        }
        let request = client.get_collections().await;
        if let Ok(_) = request {
            return Ok(());
        }
        sleep(core::time::Duration::from_secs(5)).await;
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Entity {
    id: String,
    document_type: usize,
    int_array: Vec<usize>,
    string_array: Vec<String>,
    child_objects: Vec<SubEntity>
}

#[derive(Serialize, Deserialize, Clone)]
struct SubEntity {
    id: String,
    name: String,
    number: usize
}

async fn upload_test_data(client: &AsyncSolrCloudClient, collection_name: &str) -> Result<(), Box<dyn Error>> {
    debug!("Uploading test data to Solr");
    let mut rng = rand::thread_rng();
    let data: Vec<Entity> = (0..100).map(|i| Entity {
        id: i.to_string(),
        document_type: 1,
        int_array: (0..10).map(|_| rng.gen_range(0..10)).collect(),
        string_array: (0..10).map(|_| hex::encode(rng.gen::<[u8; 16]>())).collect(),
        child_objects: (0..10).map(|j| SubEntity {
            id: format!("{i}_{j}"),
            number: rng.gen_range(0..1000),
            name: hex::encode(rng.gen::<[u8; 16]>())
        }).collect()
    }).collect();
    client.index(&UpdateQuery::new(), collection_name, &data).await?;
    Ok(())
}