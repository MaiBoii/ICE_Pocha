use axum::{
    Router, Server,Extension
};
use serde::Deserialize;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::str::FromStr;
use std::{env, net::SocketAddr};

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

    Migrator::fresh(&conn).await?;

    let app: Router = Router::new()
    .merge(routes::menu_route::menu_routes())
    .merge(routes::order_route::order_routes())
    .layer(Extension(conn));

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct TableParams{
    table_id: Option<String>
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}