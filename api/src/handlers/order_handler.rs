use axum::{Extension, response::IntoResponse, http::StatusCode, Json, extract::Query};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use chrono::Utc;
use entity::{order, order_detail};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, ActiveValue};
//use entity::{order, orders_detail, menu};
use serde::Deserialize;
use sea_orm::entity::EntityTrait;

use crate::models::order_models::CreateOrderModel;

#[derive(Debug, Deserialize)]
pub struct TableParams{
    table_id: Option<String>
}

/* ---------------------------------- 주문하기---------------------------------- */
pub async fn order_inmarket_menus(
    Query(params):Query<TableParams>, 
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(orders_detail):Json<CreateOrderModel>
) -> impl IntoResponse {
    let table_id = params.table_id.as_deref().unwrap();
    //customer_id 세션에 저장
    session.insert("customer_id", uuid::Uuid::new_v4()).unwrap();

    //order 데이터 저장
    let order_models = order::ActiveModel{
        customer_id: ActiveValue::Set(session.get::<uuid::Uuid>("customer_id").unwrap().to_string()),
        tables_id: ActiveValue::Set(table_id.to_string()),
        order_time: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let order_res = order_models.insert(&conn).await.unwrap();

    //order_detail 데이터 저장
    let order_detail_models = orders_detail.order_items.into_iter().map(|item| order_detail::ActiveModel {
        order_id: ActiveValue::Set(order_res.order_id),
        inmarket_menu_id: ActiveValue::Set(Some(item.menu_id)),
        quantity: ActiveValue::Set(item.quantity),
        ..Default::default()
    }).collect::<Vec<_>>();

    order_detail::Entity::insert_many(order_detail_models).exec(&conn).await.unwrap();

    (StatusCode::ACCEPTED, "주문이 완료되었습니다.")
}

/* -------------------------------------------------------------------------- */
