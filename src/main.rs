//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
//!
use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpRequest, HttpServer, get, post};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql;
mod server;

use graphql::schema::{create_schema, Schema};
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate dotenv;
mod database;
mod entity;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use server::token::validate_token;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:4000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn api_graphql(
    state: web::Data<AppState>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    // let token: Option<&str> = request.headers().get("token").map(|value| value.to_str().ok()).ok_or(None)?;
    let user = web::block(move || {
        // let user = token.map(|value| validate_token(&value))?;
        let context = graphql::schema::Context{
            conn: state.pool.get().expect("couldn't get db connection from pool"),
            user: None,
        };
        let res = data.execute(&state.schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

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

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(api_graphql)
            .service(graphiql)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}