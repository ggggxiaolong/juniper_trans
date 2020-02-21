//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
//!
use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, get, post};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql;
mod app;

use graphql::schema::{create_schema, Schema};
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate juniper;
extern crate dotenv;

mod database;
mod entity;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use app::session::Session;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:4000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/test")]
async fn test() -> HttpResponse {
    let req = reqwest::get("https://httpbin.org/ip").await;
    let text = match req{
        Ok(body) => {
            if let Ok(text) = body.text().await {
                text
            } else { "error body".to_owned()}
        },
        Err(e) => { format!("{:?}", e) },
    };
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(&text)
}

#[post("/graphql")]
async fn api_graphql(
    state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let context = graphql::schema::Context{
        // conn: state.pool.get().expect("couldn't get db connection from pool"),
        conn: state.pool.clone(),
        user: session.user,
    };
    let res = data.execute_async(&state.schema, &context).await;
    // Ok::<_, serde_json::error::Error>(?)
    let body = serde_json::to_string(&res)?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

fn establish_connection() -> DbPool {
    dotenv().ok();
    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[derive(Clone)]
struct AppState{
    pool: DbPool,
    schema: Arc<Schema>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());
    let app_state = AppState{
        pool: establish_connection(),
        schema: schema,
    };
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(api_graphql)
            .service(graphiql)
            .service(test)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}