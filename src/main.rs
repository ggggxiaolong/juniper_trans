//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
//!
use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpServer, HttpResponse};
// use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod app;
mod graphql;

use graphql::schema::{create_schema, Schema};
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate dotenv;
extern crate juniper;

mod database;
mod entity;

use app::session::Session;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// #[get("/graphql")]
// async fn graphiql() -> HttpResponse {
//     let html = graphiql_source("http://localhost:4000/graphql");
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(html)
// }

// #[post("/graphql")]
async fn api_graphql(
    state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let context = graphql::schema::Context {
        conn: state.pool.clone(),
        user: session.user,
    };
    println!("/graphql");
    let res = data.execute_async(&state.schema, &context).await;
    // Ok::<_, serde_json::error::Error>(?)
    let body = serde_json::to_string(&res)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

// #[get("/test")]
// async fn test_api() -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body("ok"))
// }

// #[post("/test2")]
// async fn test_api2(data: web::Json<GraphQLRequest>,) -> Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body("ok"))
// }

fn establish_connection() -> DbPool {
    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[derive(Clone)]
struct AppState {
    pool: Arc<DbPool>,
    schema: Arc<Schema>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    // Create Juniper schema
    let schema = Arc::new(create_schema());
    let app_state = AppState {
        pool: Arc::new(establish_connection()),
        schema: schema,
    };
    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["OPTION", "POST", "GET"])
                    .max_age(3600)
                    .finish(),
            )
            // .data(web::PayloadConfig::new(1024 * 1024 * 50))
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            .data(app_state.clone())
            .service(
                web::resource("/graphql")
                    .app_data(web::JsonConfig::default().limit(1024 * 1024 * 50))
                    .route(web::post().to(api_graphql)),
            )
        // .service(test_api)
        // .service(test_api2)
    })
    .bind(bind_address)?
    .run()
    .await
}
