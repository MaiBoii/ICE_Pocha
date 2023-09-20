use axum::{
    Router, Server, Json, routing::{get, post}, http::StatusCode,response::IntoResponse
};
use axum::extract::{State, Path, Query};
use icepocha_service::sea_orm::{Database,DatabaseConnection};
use serde::Deserialize;
use entity::*;
use migration::{Migrator, MigratorTrait, Table};
use models::{MenuStruct, OrdersDetailStruct};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, Order};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use chrono::Utc;

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

    Migrator::fresh(&conn).await?;

    let state = ConnState {conn };

    let app = Router::new()
    .route("/", get(root))
    .route("/menu", get(show_menus).post(ordering))
    //.route("/order", post(send_orders_detail))
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

//주문 정보 생성
async fn ordering(Query(params):Query<TableParams>, 
                state: State<ConnState>, 
            ) 
    -> impl IntoResponse {
    let table_id = params.table_id.as_deref().unwrap();
    let order_model = order::ActiveModel {
        tables_id: Set(table_id.to_owned()),
        ordered_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    order_model.insert(&state.conn).await.unwrap();
    (StatusCode::ACCEPTED, "주문 정보가 생성되었습니다.")
}

// //OrdersDetail 정보 생성
// async fn send_orders_detail(
//         Query(params):Query<TableParams>, 
//         state: State<ConnState>, 
//         Json(orders_detail):Json<OrdersDetailStruct>
//     ) 
//     -> impl IntoResponse {
    
//     let table_id = params.table_id.as_deref().unwrap();
//     let order_model = order::ActiveModel {
//         tables_id: Set(table_id.to_owned()),
//         ordered_at: Set(Utc::now().naive_utc()),
//         ..Default::default()
//     };

//     let add_order = order_model.insert(&state.conn).await.unwrap();

//     let orders_detail_model = orders_detail::ActiveModel {
//         order_id: Set(add_order.order_id.to_owned()),
//         menu_id: Set(.menu_id.to_owned()),
//         quantity: Set(orders_detail.quantity.to_owned()),
//         //price = quantity * menu.price
//         price: Set(),
//         requests: Set(orders_detail.requests.to_owned()),
//         ..Default::default()
//     };
//     orders_detail_model.insert(&state.conn).await.unwrap();
//     (StatusCode::ACCEPTED, "주문이 완료되었습니다.")
// }

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}