use std::fmt::{Debug, Display, Formatter};
use rocket::form::{FromForm};
use rocket::{get, Request, response, Response, serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, OpenApiError, swagger_ui::*};
use serde::{Deserialize, Serialize};
use reqwest::{Client};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::response::OpenApiResponderInner;
use crate::schemars::Map;

//const HOST: &str = "http://varnish";
const HOST: &str = "http://localhost:25900";

#[derive(Serialize, Deserialize, JsonSchema)]
struct SolrResponse {
    response: SolrDataResponse
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct SolrDataResponse {
    docs: Vec<Entity>
}


#[derive(Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, JsonSchema)]
struct SubEntity {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<i32>,
}


#[openapi]
#[get("/hello_world")]
/// # Hello World
///
/// Returns Hello World
fn hello_world() -> &'static str {
    "Hello World"
}


#[derive(Serialize, Deserialize, JsonSchema, FromForm)]
struct Info {
    /// Some example values: <ul><li><code>1</code></li></ul>
    // Required to be both Optional<T> for route to match even if not given, and #[schemars(required)] to show as required in swagger ui
    #[schemars(required)]
    document_type: Option<i32>,
}

pub struct BadRequest {
    message: String
}
impl OpenApiResponderInner for BadRequest {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "400".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "Bad Request".to_string(),
                ..Default::default()
            }),
        );
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}

impl Debug for BadRequest {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for BadRequest {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for BadRequest {}

impl<'r> Responder<'r, 'static> for BadRequest {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = self.message;
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::Plain)
            .status(Status::new(400))
            .ok()
    }
}

#[openapi]
#[get("/json_serialization?<info..>")]
/// # Json Serialization
///
/// Serializing a json document
async fn json_serialization(info: Info, client: &State<Client>) -> Result<Json<Vec<Entity>>, BadRequest> {
    let document_type = match info.document_type {
        Some(ret) => ret,
        None => {return Err(BadRequest {message: "Required parameter not found or incorrect: document_type".to_string()})}
    };
    let url: String = format!("{host}/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:{document_type}", host=HOST, document_type=document_type);
    let solr = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<SolrResponse>()
        .await
        .unwrap();
    Ok(Json(solr.response.docs))
}

#[openapi]
#[get("/anonymization")]
/// # Anonymization
///
/// Serializing a json document
async fn anonymization(client: &State<Client>) -> Json<Vec<Entity>> {
    let url: String = format!("{host}/solr/performance/select?fl=id,document_type,int_array,string_array,child_objects,name,number,[child]&q=*:*&rows=100&fq=document_type:1", host=HOST);
    let mut solr = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<SolrResponse>()
        .await
        .unwrap();
    for doc in solr.response.docs.iter_mut() {
        for child in doc.child_objects.as_mut().unwrap().iter_mut() {
            if child.number.unwrap() < 100 {
                child.number = Option::from(0);
            }
        }
    }
    Json(solr.response.docs)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .manage(Client::new())
        .mount(
            "/",
            openapi_get_routes![
                hello_world,
                json_serialization,
                anonymization
            ],
        )
        .mount(
            "/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await.expect("Could not start rocket");
}