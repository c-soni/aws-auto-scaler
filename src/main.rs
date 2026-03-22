pub mod utils;
use std::string::String;
use serde::{Deserialize, Serialize};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use redis::aio::MultiplexedConnection;
use crate::utils::redis::{get_connection, get_string, set_string};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

async fn do_stuff(connection: MultiplexedConnection) {
    set_string(connection.clone(), "foo", "bar").await;

    match get_string(connection.clone(), "foo").await {
        Ok(s) => log::info!("Received: {}", s.unwrap_or(String::from("EMPTY"))),
        Err(_) => log::error!("ERROR"),
    };

    match get_string(connection.clone(), "foo2").await {
        Ok(s) => log::info!("Received: {}", s.unwrap_or(String::from("EMPTY"))),
        Err(_) => log::error!("ERROR"),
    };
}

/// This handler uses json extractor
async fn index(item: web::Json<MyObj>) -> HttpResponse {
    log::info!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    log::info!("request: {req:?}");
    log::info!("model: {item:?}");

    HttpResponse::Ok().json(item.0) // <- send json response
}

/// This handler manually load request payload and parse json object
async fn index_manual(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let connection: MultiplexedConnection = get_connection().await.unwrap();
    do_stuff(connection).await;

    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/extractor").route(web::post().to(index)))
            .service(
                web::resource("/extractor2")
                    .app_data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(extract_item)),
            )
            .service(web::resource("/manual").route(web::post().to(index_manual)))
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // Connect to S3
    // Connect to EC2 and create min instances
    // Connect to SQS
    // Polling for /status endpoint with image reqId query parameter
    // POST endpoint for image processing request
}
