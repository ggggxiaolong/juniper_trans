//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
//!
use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{ middleware, post, web, App, Error, HttpResponse, HttpServer };
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

#[post("/graphql")]
async fn api_graphql(
    state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    println!("{}", "api_graphql");
    let context = graphql::schema::Context {
        conn: state.pool.clone(),
        user: session.user,
    };
    let res = data.execute_async(&state.schema, &context).await;
    // Ok::<_, serde_json::error::Error>(?)
    let body = serde_json::to_string(&res)?;
    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .content_type("application/json")
        .body(body))
}

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
    pool: DbPool,
    schema: Arc<Schema>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());
    let app_state = AppState {
        pool: establish_connection(),
        schema: schema,
    };
    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["OPTION", "POST", "GET"])
                    .max_age(3600)
                    .finish()
            )
            .data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(api_graphql)
    })
    .bind(bind_address)?
    .run()
    .await
}
