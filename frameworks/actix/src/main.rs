use actix_web::web::{Data, Query};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use awc::Client;
use serde::{Deserialize, Serialize};
use std::str;
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize)]
struct SolrResponse {
    response: SolrDataResponse,
}

#[derive(Serialize, Deserialize)]
struct SolrDataResponse {
    docs: Vec<Entity>,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct Entity {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_array: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    int_array: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_objects: Option<Vec<SubEntity>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct SubEntity {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i32>,
}

#[utoipa::path(responses(
    (status = 200, description = "Returns Hello World", body = String)
))]
#[get("/hello_world")]
async fn hello_world(_client: Data<Client>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello, world!")
}

#[derive(Serialize, Deserialize, IntoParams)]
struct Info {
    /// Some example values: <ul><li><code>1</code></li></ul>
    document_type: i32,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Serializing a json document", body = [Entity]),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    ),
    params(
        Info
    )
)]
#[get("/json_serialization")]
async fn json_serialization(
    info: Query<Info>,
    client: Data<Client>,
    config: Data<Config>,
) -> impl Responder {
    let url: String = format!("{host}/solr/test_data/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:{document_type}", host=config.benchmark_host, document_type=info.document_type);
    let solr = client.get(url).send().await;
    let mut solr = match solr {
        Ok(solr) => solr,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to fetch Solr data: {}", e));
        }
    };
    let solr = solr
        //Set limit to 2MB
        .json::<SolrResponse>()
        .limit(2000000)
        .await;
    let solr = match solr {
        Ok(solr) => solr,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to parse Solr response: {}", e));
        }
    };
    HttpResponse::Ok().json(solr.response.docs)
}

#[utoipa::path(
responses(
    (status = 200, description = "Doing data processing on a json document", body = [Entity]),
    (status = 400, description = "Bad Request", body = String),
    (status = 500, description = "Internal Server Error", body = String)
))]
#[get("/anonymization")]
async fn anonymization(client: Data<Client>, config: Data<Config>) -> impl Responder {
    let url: String = format!("{host}/solr/test_data/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1", host=config.benchmark_host);
    let solr = client.get(url).send().await.map_err(|e| {
        HttpResponse::InternalServerError().body(format!("Failed to fetch Solr data: {}", e))
    });
    let mut solr = match solr {
        Ok(solr) => solr,
        Err(e) => return e,
    };
    let solr = solr
        //Set limit to 2MB
        .json::<SolrResponse>()
        .limit(2000000)
        .await
        .map_err(|e| {
            HttpResponse::InternalServerError()
                .body(format!("Failed to parse Solr response: {}", e))
        });
    let mut solr = match solr {
        Ok(solr) => solr,
        Err(e) => return e,
    };
    for doc in solr.response.docs.iter_mut() {
        for child in doc.child_objects.as_mut().unwrap().iter_mut() {
            if child.number.unwrap() < 100 {
                child.number = Option::from(0);
            }
        }
    }
    HttpResponse::Ok().json(solr.response.docs)
}

pub struct Config {
    pub benchmark_host: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            benchmark_host: std::env::var("BENCHMARK_HOST")
                .unwrap_or("http://localhost:8983".to_string()),
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(hello_world, json_serialization, anonymization),
    components(schemas(Entity, SubEntity),)
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Client::default()))
            .app_data(Data::new(Config::default()))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/openapi.json", openapi.clone()))
            .service(hello_world)
            .service(json_serialization)
            .service(anonymization)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
