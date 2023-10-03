use axum::{Extension, response::IntoResponse, http::StatusCode, Json, extract::Query};
use axum_sessions::extractors::WritableSession;
use chrono::Utc;
use entity::{order, order_detail, inmarket_menu, packaged_menu, date_margin};
use sea_orm::{DatabaseConnection, ActiveModelTrait, ActiveValue,DbErr, QueryFilter, ColumnTrait, Set};
use serde::Deserialize;
use sea_orm::entity::EntityTrait;

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

    if let Some(customer_id) = session.get::<uuid::Uuid>("customer_id") {
        // customer_id가 이미 존재하는 경우 그냥 넘어감
        println!("customer_id: {}", customer_id);
    } else {
        session.insert("customer_id", uuid::Uuid::new_v4()).unwrap();
    }

    // order 데이터 저장
    //if same session in same table, just update order_time
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
            inmarket_menu_id: ActiveValue::Set(Some(item.menu_id)),
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


/* ------------------------------- 포장 메뉴 주문 함수 ------------------------------ */
pub async fn order_packaged_menus(
    Query(params): Query<TableParams>,
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(orders_detail): Json<CreateOrderModel>,
) -> impl IntoResponse {

    let table_id = params.table_id.as_deref().unwrap();

    if let Some(customer_id) = session.get::<uuid::Uuid>("customer_id") {
        // customer_id가 이미 존재하는 경우 그냥 넘어감
        println!("customer_id: {}", customer_id);
    } else {
        session.insert("customer_id", uuid::Uuid::new_v4()).unwrap();
    }

    // order 데이터 저장
    //if same session in same table, just update order_time
    let order_models = order::ActiveModel {
        customer_id: ActiveValue::Set(session.get::<uuid::Uuid>("customer_id").unwrap().to_string()),
        tables_id: ActiveValue::Set(table_id.to_string()),
        order_time: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let order_res = order_models.insert(&conn).await.unwrap();

    // get_menu_price_margin 함수 정의
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


/* ------------------------------- 결제 완료 함수 ------------------------------ */
pub async fn payment_complete(
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    
    //find session's customer_id
    let customer_id = session.get::<uuid::Uuid>("customer_id").unwrap();
    println!("customer_id: {}", customer_id);

    //find all order_id with customer_id
    let order_id = order::Entity::find()
        .filter(order::Column::CustomerId.contains(customer_id.to_string()))
        .all(&conn)
        .await
        .unwrap()
        .into_iter()
        .map(|item| item.order_id)
        .collect::<Vec<i32>>();

    //find all order_detail_id with order_id and update completed
    for id in order_id {
        let order_detail_id = order_detail::Entity::find()
            .filter(order_detail::Column::OrderId.contains(id.to_string()))
            .all(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|item| item.order_detail_id)
            .collect::<Vec<i32>>();

        for detail_id in order_detail_id {
            let order_detail_model = order_detail::ActiveModel {
                order_detail_id: ActiveValue::Set(detail_id),
                completed: ActiveValue::Set(1),
                ..Default::default()
            };

            order_detail_model.update(&conn).await.unwrap();
        }
    }
    session.remove("customer_id");
    println!("세션이 만료되었습니다.");
}
/* -------------------------------------------------------------------------- */


/* -------------------------------- 매출 정리 함수 -------------------------------- */
pub async fn shop_closing(
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    // 매장 영업 종료 시, 매출 정리 함수
    //find all order_details where completed = 1
    let order_detail_id = order_detail::Entity::find()
        .filter(order_detail::Column::Completed.contains("1"))
        .all(&conn)
        .await
        .unwrap()
        .into_iter()
        .map(|item| item.order_detail_id)
        .collect::<Vec<i32>>();
    //sum of total_margin
    let mut _total_margin = 0;
    for id in order_detail_id {
        let order_detail = order_detail::Entity::find_by_id(id)
            .one(&conn)
            .await
            .unwrap()
            .unwrap();
        _total_margin += order_detail.total_margin;
    }

    let todays_margin = date_margin::ActiveModel {
        profit_margin: Set(_total_margin.to_owned()),
        ..Default::default()
    };

    let _ = todays_margin.insert(&conn).await;

    (StatusCode::ACCEPTED, "일자가 변경되었습니다.")
}

/* -------------------------------------------------------------------------- */