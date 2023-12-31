use axum::{Extension, response::IntoResponse, http::StatusCode, Json, extract::Query};
use axum_sessions::extractors::WritableSession;
use chrono::Utc;
use entity::{order, order_detail, inmarket_menu, packaged_menu};
use sea_orm::{DatabaseConnection, ActiveModelTrait, ActiveValue,DbErr};
use serde::Deserialize;
use sea_orm::entity::EntityTrait;
use tungstenite::{connect, Message};
use url::Url;

use crate::models::order_models::CreateOrderModel;

#[derive(Debug, Deserialize)]
pub struct TableParams{
    table_id: Option<String>
}

/* ------------------------------- 매장 메뉴 주문 함수 ------------------------------ */
pub async fn order_inmarket_menus(
    Query(params): Query<TableParams>,
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(orders_detail): Json<CreateOrderModel>,
) -> impl IntoResponse {

    let table_id = params.table_id.as_deref().unwrap();

    if let Some(_customer_id) = session.get::<uuid::Uuid>("customer_id") {
    } else {
        session.insert("customer_id", uuid::Uuid::new_v4()).unwrap();
    }

    // order 데이터 저장
    let order_models = order::ActiveModel {
        customer_id: ActiveValue::Set(session.get::<uuid::Uuid>("customer_id").unwrap().to_string()),
        tables_id: ActiveValue::Set(table_id.to_string()),
        order_time: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let order_res = order_models.insert(&conn).await.unwrap();

    // get_menu_price_margin 함수 정의
    async fn get_menu_price_margin(menu_id: i32, conn: &DatabaseConnection) -> Result<(i32,i32), DbErr> {
        let menu_price = inmarket_menu::Entity::find_by_id(menu_id)
            .one(conn)
            .await
            .unwrap()
            .unwrap();

        Ok((menu_price.price, menu_price.profit_margin))
    }

    // 각 주문 디테일 레코드를 개별적으로 추가
    for item in &orders_detail.order_items {
        let menu_price_margin = get_menu_price_margin(item.menu_id, &conn).await.unwrap();
        let quantity = item.quantity;

        let order_detail_model = order_detail::ActiveModel {
            order_id: ActiveValue::Set(order_res.order_id),
            customer_id:ActiveValue::Set(session.get::<uuid::Uuid>("customer_id").unwrap().to_string()),
            inmarket_menu_id: ActiveValue::Set(Some(item.menu_id)),
            quantity: ActiveValue::Set(quantity),
            sub_total_price: ActiveValue::Set(menu_price_margin.0 * quantity),
            total_margin: ActiveValue::Set(menu_price_margin.1 * quantity),
            ..Default::default()
        };

        order_detail_model.insert(&conn).await.unwrap();
    }

    let (mut socket, response) =
        connect(Url::parse("ws://127.0.0.1:8080/socket").unwrap()).expect("Can't connect");

    socket.send(Message::Text("Refresh The Page".into())).unwrap();

    println!("{}번 테이블 손님이 주문하셨습니다.",table_id);
    
    (StatusCode::ACCEPTED, "주문이 완료되었습니다.")
}
/* -------------------------------------------------------------------------- */

/* ------------------------------- 포장 메뉴 주문 함수 ------------------------------ */
pub async fn order_packaged_menus(
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(orders_detail): Json<CreateOrderModel>,
) -> impl IntoResponse {

    if let Some(customer_id) = session.get::<uuid::Uuid>("customer_id") {
        println!("customer_id: {}", customer_id);
    } else {
        session.insert("customer_id", uuid::Uuid::new_v4()).unwrap();
    }

    let table_id = "takeout";

    // order 데이터 저장
    let order_models = order::ActiveModel {
        customer_id: ActiveValue::Set(session.get::<uuid::Uuid>("customer_id").unwrap().to_string()),
        tables_id: ActiveValue::Set(table_id.to_string()),
        order_time: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let order_res = order_models.insert(&conn).await.unwrap();

    async fn get_menu_price_margin(menu_id: i32, conn: &DatabaseConnection) -> Result<(i32,i32), DbErr> {
        let menu_price = packaged_menu::Entity::find_by_id(menu_id)
            .one(conn)
            .await
            .unwrap()
            .unwrap();

        Ok((menu_price.price, menu_price.profit_margin))
    }

    // 각 주문 디테일 레코드를 개별적으로 추가
    for item in &orders_detail.order_items {
        let menu_price_margin = get_menu_price_margin(item.menu_id, &conn).await.unwrap();
        let quantity = item.quantity;

        let order_detail_model = order_detail::ActiveModel {
            order_id: ActiveValue::Set(order_res.order_id),
            customer_id:ActiveValue::Set(session.get::<uuid::Uuid>("customer_id").unwrap().to_string()),
            packaged_menu_id: ActiveValue::Set(Some(item.menu_id)),
            quantity: ActiveValue::Set(quantity),
            sub_total_price: ActiveValue::Set(menu_price_margin.0 * quantity),
            total_margin: ActiveValue::Set(menu_price_margin.1 * quantity),
            ..Default::default()
        };

        order_detail_model.insert(&conn).await.unwrap();
    }

    (StatusCode::ACCEPTED, "주문이 완료되었습니다.")
}
/* -------------------------------------------------------------------------- */

