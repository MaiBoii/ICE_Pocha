use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Html,
    routing::{get, get_service, post},
    Router, Server,
};
use icepocha_service::sea_orm::{Database, DatabaseConnection};
use entity::{menu, order, orders_detail};
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tera::Tera;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { templates, conn };

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

#[derive(Clone)]
struct AppState {
    templates: Tera,
    conn: DatabaseConnection,
}

#[derive(Deserialize)]
struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}

async fn create_menu(
    state: State<AppState>,
    mut cookies: Cookies,
    form: Form<post::Model>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    let form = form.0;

    MutationCore::create_post(&state.conn, form)
        .await
        .expect("could not insert post");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "Post succcessfully added".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}