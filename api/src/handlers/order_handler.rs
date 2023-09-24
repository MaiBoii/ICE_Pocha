use axum::{Extension, response::IntoResponse, http::StatusCode, Json, extract::Query};
use axum_sessions::extractors::WritableSession;
use chrono::Utc;
use entity::{order, order_detail, inmarket_menu};
use sea_orm::{DatabaseConnection, ActiveModelTrait, ActiveValue, QueryFilter, ColumnTrait, ActiveModelBehavior};
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

    println!("{:}", session.get::<uuid::Uuid>("customer_id").unwrap().to_string());

    let order_res = order_models.insert(&conn).await.unwrap();

    //order_detail 데이터 저장
    // let order_detail_models = orders_detail
    //                     .order_items
    //                     .into_iter()
    //                     .map(|item| {
    //                             let menu_price = get_menu_price(item.menu_id, &conn).await.unwrap(); // 메뉴 가격을 가져오는 함수
    //                             let quantity = item.quantity;

    //                             let sub_total_price = menu_price * quantity;
    //                             let total_margin = calculate_margin(sub_total_price);
    //                         order_detail::ActiveModel {

    //     order_id: ActiveValue::Set(order_res.order_id),
    //     inmarket_menu_id: ActiveValue::Set(Some(item.menu_id)),
    //     quantity: ActiveValue::Set(item.quantity),
    //     sub_total_price: ActiveValue::Set(0),
    //     total_margin: ActiveValue::Set(0),
    //     ..Default::default()
    //                         }
    // }).collect::<Vec<_>>();

    let order_detail_models = orders_detail
    .order_items
    .into_iter()
    .map(|item| {

        



        let quantity = item.quantity;

        let sub_total_price = menu_price * quantity;

        order_detail::ActiveModel {
            order_id: ActiveValue::Set(order_res.order_id),
            inmarket_menu_id: ActiveValue::Set(Some(item.menu_id)),
            quantity: ActiveValue::Set(quantity),
            sub_total_price: ActiveValue::Set(sub_total_price),
            total_margin: ActiveValue::Set(0),
            ..Default::default()
        }
    })
    .collect::<Vec<_>>();

    order_detail::Entity::insert_many(order_detail_models).exec(&conn).await.unwrap();
    
    (StatusCode::ACCEPTED, "주문이 완료되었습니다.")
}

/* -------------------------------------------------------------------------- */
