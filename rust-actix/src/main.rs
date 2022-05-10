use std::fmt::{Debug, Display, Formatter};
use actix_web::{App, HttpServer, ResponseError};
use actix_web::client::Client;
use paperclip::actix::{
    OpenApiExt, Apiv2Schema, api_v2_operation, api_v2_errors,
    web::{self},
};
use serde::{Serialize, Deserialize};
use std::str;
use actix_web::error::ErrorBadRequest;
use paperclip::actix::web::{HttpResponse, Json};

const HOST: &str = "http://varnish";
//const HOST: &str = "http://localhost:25900";

#[derive(Serialize, Deserialize, Apiv2Schema)]
struct SolrResponse {
    response: SolrDataResponse
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
struct SolrDataResponse {
    docs: Vec<Entity>
}


#[derive(Serialize, Deserialize, Apiv2Schema)]
struct Entity {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_array: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    int_array: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    child_objects: Option<Vec<SubEntity>>
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
struct SubEntity {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i32>,
}

#[api_v2_operation(
    produces = "text/plain",
    summary = "Returns Hello World"
)]
async fn hello_world(_client: web::Data<Client>) -> Result<String, ()> {
    Ok("Hello World".to_owned())
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
struct Info {
    /// Some example values: <ul><li><code>1</code></li></ul>
    document_type: i32,
}
#[api_v2_errors(code=400, description = "Bad request")]
pub struct BadRequest {}

impl Debug for BadRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for BadRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ResponseError for BadRequest {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::from_error(ErrorBadRequest("Bad Request"))
    }
}
#[api_v2_operation(summary = "Serializing a json document")]
async fn json_serialization(info: web::Query<Info>, client: web::Data<Client>) -> Result<Json<Vec<Entity>>, BadRequest> {
    let url: String = format!("{host}/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:{document_type}", host=HOST, document_type=info.document_type);
    let solr = client
        .get(url)    // <- Create request builder
        .send()
        .await
        .unwrap()
        //Set limit to 2MB
        .json::<SolrResponse>().limit(2000000)
        .await
        .unwrap();
    Ok(Json(solr.response.docs))
}

#[api_v2_operation(
    summary = "Serializing a json document",
)]
async fn anonymization(client: web::Data<Client>) -> Result<Json<Vec<Entity>>, ()> {
    let url: String = format!("{host}/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1", host=HOST);
    let mut solr = client
        .get(url)    // <- Create request builder
        .send()
        .await
        .unwrap()
        //Set limit to 2MB
        .json::<SolrResponse>().limit(2000000)
        .await
        .unwrap();
    for doc in solr.response.docs.iter_mut() {
        for child in doc.child_objects.as_mut().unwrap().iter_mut() {
            if child.number.unwrap() < 100 {
                child.number = Option::from(0);
            }
        }
    }
    Ok(Json(solr.response.docs))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .data(Client::default())
        .wrap_api()
        .service(
            web::resource("/hello_world")
                .route(web::get().to(hello_world))
        )
        .service(
            web::resource("/json_serialization")
                .route(web::get().to(json_serialization))
        )
        .service(
            web::resource("/anonymization")
                .route(web::get().to(anonymization))
        )
        .with_json_spec_at("/api/spec")
        .with_swagger_ui_at("")

        .build()
    ).bind("0.0.0.0:8080")?
    .run().await
}
