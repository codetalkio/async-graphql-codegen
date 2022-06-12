#![feature(never_type)]
#![feature(default_free_fn)]

mod model;

use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_std::sync::{Arc, RwLock};

use crate::model::Query;
use std::error::Error;
use std::fs::File;

type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("localhost:1234")?
    .run()
    .await
    .map_err(|e| e.into())
}
