use std::error::Error;
use std::fs::{File};
use std::io::{Read, Seek, Write};
use std::path::Path;
use chrono::Duration;
use log::debug;
use serde::{Serialize, Deserialize};
use zip::write::FileOptions;
use walkdir::{DirEntry, WalkDir};
use tempfile::tempfile;
use tokio::time::sleep;
use rand::Rng;

const CONFIG_NAME: &str = "test_data";
const COLLECTION_NAME: &str = "test_data";

#[derive(Serialize, Deserialize, Clone)]
pub struct SolrResponse {
    #[serde(rename = "responseHeader")]
    pub response_header: SolrResponseHeader,
    pub error: Option<SolrResponseError>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolrResponseError {
    pub msg: Option<String>,
    pub trace: Option<String>,
    pub code: usize
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SolrResponseHeader {
    pub status: usize,
    #[serde(rename = "QTime")]
    pub q_time: usize
}


// https://github.com/zip-rs/zip/blob/e32db515a2a4c7d04b0bf5851912a399a4cbff68/examples/write_dir.rs
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<(), Box<dyn Error>>
    where
        T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix)?;
        if path.is_file() {
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

pub async fn upload_data_to_solr(host: &str) -> Result<(), Box<dyn Error>> {
    wait_for_solr(host, Duration::seconds(30)).await?;
    upload_config(host, CONFIG_NAME, Path::new("benchmark_server/solrconfig")).await?;
    create_collection(host, COLLECTION_NAME, CONFIG_NAME, 1, 1).await?;
    upload_test_data(host, COLLECTION_NAME).await?;
    Ok(())
}

async fn wait_for_solr(host: &str, timeout: Duration) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let until = chrono::Local::now() + timeout;
    debug!("Waiting {timeout} for solr");
    loop {
        if until < chrono::Local::now() {
            return Err(format!("Could not reach solr within timeout").into());
        }
        let request = client
            .get(format!("{host}/solr/admin/collections?action=CLUSTERSTATUS"));
        match request.send().await {
            Ok(solr_response) => { match solr_response.json::<SolrResponse>().await {
                Ok(solr_response) => {
                    if solr_response.response_header.status == 0 {
                        return Ok(())
                    }
                }
                Err(_) => {}
            } }
            Err(_) => {}
        }
        sleep(core::time::Duration::from_secs(5)).await;
    }
}

async fn upload_config(host: &str, name: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    debug!("Uploading config to Solr");
    let client = reqwest::Client::new();
    let mut request = client
        .post(format!("{host}/solr/admin/configs?action=UPLOAD&name={name}"))
        .header("Content-Type", "application/octet-stream");
    let mut outfile = tempfile()?;
    path.try_exists()?;
    if path.is_dir() {
        let walkdir = WalkDir::new(path);
        let it = walkdir.into_iter();
        zip_dir(&mut it.filter_map(|e| e.ok()), path, &outfile, zip::CompressionMethod::Stored)?;
        outfile.rewind()?;
    }
    else {
        outfile = File::open(path)?;
    }
    let mut vec = Vec::new(); outfile.read_to_end(&mut vec)?;
    request = request.body(vec);
    let solr_response = request.send().await?.json::<SolrResponse>().await?;
    match solr_response.response_header.status {
        0 => {Ok(())}
        _ => {Err(format!("Could not upload config\n {:?}", solr_response.error.unwrap()).into())}
    }
}

async fn create_collection(host: &str, name: &str, config: &str, shards: usize, replication_factor: usize) -> Result<(), Box<dyn Error>> {
    debug!("Creating solr collection");
    let client = reqwest::Client::new();
    let request = client.get(format!("{host}/solr/admin/collections?action=CREATE&name={name}&numShards={shards}&replicationFactor={replication_factor}&collection.configName={config}"));
    let solr_response = request.send().await?.json::<SolrResponse>().await?;
    match solr_response.response_header.status {
        0 => {Ok(())}
        _ => {Err(format!("Could not create collection\n {:?}", solr_response.error.unwrap()).into())}
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

async fn upload_test_data(host: &str, collection: &str) -> Result<(), Box<dyn Error>> {
    debug!("Uploading test data to Solr");
    let mut data: Vec<Entity> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        let id = i.to_string();
        data.push(Entity {
            id: id.to_string(),
            document_type: 1,
            int_array: (0..10).map(|_| rng.gen_range(0..10)).collect(),
            string_array: (0..10).map(|_| hex::encode(rng.gen::<[u8; 16]>())).collect(),
            child_objects: (0..10).map(|j| SubEntity {
                id: format!("{id}_{j}").to_string(),
                number: rng.gen_range(0..1000),
                name: hex::encode(rng.gen::<[u8; 16]>())
            }).collect()
        });
    }
    let client = reqwest::Client::new();
    let request = client.get(format!("{host}/solr/{collection}/update?commit=true&overwrite=true&wt=json")).json(&data);
    let solr_response = request.send().await?.json::<SolrResponse>().await?;
    match solr_response.response_header.status {
        0 => {Ok(())}
        _ => {Err(format!("Could not upload data\n {:?}", solr_response.error.unwrap()).into())}
    }
}