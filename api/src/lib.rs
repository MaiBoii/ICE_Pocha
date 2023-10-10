use axum::{
    Router, Server,Extension
};
use axum_sessions::{
    async_session::MemoryStore,
    SessionLayer,
};
use migration::Migrator;
use sea_orm::*;
use tower_http::cors::{CorsLayer, Any};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use sea_orm_migration::prelude::*;
use rand::Rng;

mod models;
mod routes;
mod handlers;
mod utils;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    //env::set_var("RUST_LOG", "debug");
    //tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let db_url = (*utils::constants::DATABASE_URL).clone();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url).await.expect("Failed to connect to db");

    // //새로 고치는 거
    Migrator::fresh(&conn).await?;

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    
    let store = MemoryStore::new();
    let mut rng = rand::thread_rng();
    let mut secret: [u8; 64] = [0; 64];

    for i in 0..64 {
        secret[i] = rng.gen::<u8>();
    }

    let session_layer = SessionLayer::new(store, &secret).with_secure(false);
    
    let app: Router = Router::new()
    .merge(routes::menu_route::menu_routes())
    .merge(routes::order_route::order_routes())
    .merge(routes::admin_route::admin_routes())
    .layer(cors)
    .layer(Extension(conn))
    .layer(session_layer);

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