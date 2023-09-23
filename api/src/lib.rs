use axum::{
    Router, Server,Extension
};
use migration::{Migrator};
use sea_orm::*;
use std::str::FromStr;
use std::{env, net::SocketAddr};
use sea_orm_migration::prelude::*;

mod models;
mod routes;
mod handlers;
mod utils;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let db_url = (*utils::constants::DATABASE_URL).clone();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url).await.expect("Failed to connect to db");

    // //새로 고치는 거
    Migrator::refresh(&conn).await?;

    let app: Router = Router::new();
    // .merge(routes::menu_route::menu_routes())
    // .merge(routes::order_route::order_routes())
    // .layer(Extension(conn));

    //IP 연결
    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}