use axum::{
    Router, Server, Json, routing::{get, post}, http::StatusCode,response::IntoResponse
};
use axum::extract::{State, Path, Query};
use icepocha_service::sea_orm::{Database,DatabaseConnection};
use serde::Deserialize;
use entity::*;
use migration::{Migrator, MigratorTrait, Table};
use models::{MenuStruct, OrderStruct};
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use chrono::{Utc};

mod models;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    Migrator::refresh(&conn).await?;

    let state = ConnState {conn };

    let app = Router::new()
    .route("/", get(root))
    .route("/menu", get(show_menus))
    .route("/menu/:table_id", post(ordering))
    .with_state(state);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Clone)]
struct ConnState {
    conn: DatabaseConnection,
}

#[derive(Debug, Deserialize)]
struct TableParams{
    table_id: Option<String>
}

//root
async fn root() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "아이스 포장마차 서버입니다.")
}

//메뉴 보이기
async fn show_menus(state: State<ConnState>) -> impl IntoResponse {
    let menus: Vec<MenuStruct> = entity::menu::Entity::find().all(&state.conn).await.unwrap().into_iter().map(|item| MenuStruct {
        menu_id: item.menu_id,
        name: item.name.to_owned(),
        price: item.price,
    }).collect();
    
    (StatusCode::ACCEPTED,Json(menus))
}

//주문 넣기
async fn ordering(Query(params):Query<TableParams>, state: State<ConnState>, Json(order): Json<OrderStruct>) -> impl IntoResponse {
    let table_id = params.table_id.as_deref().unwrap();
    let order_model = order::ActiveModel {
        tables_id: Set(table_id.to_owned()),
        ordered_at: Set(Utc::now().naive_utc()),
        total_price: Set(order.total_amount.to_owned()),
        ..Default::default()
    };
    order_model.insert(&state.conn).await.unwrap();
    (StatusCode::ACCEPTED, "주문이 완료되었습니다.")
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}