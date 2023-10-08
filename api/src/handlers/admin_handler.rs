use axum::{Json, http::StatusCode, response::{IntoResponse,Redirect}, Extension};
use axum_sessions::extractors::{WritableSession, ReadableSession};
use entity::{order, order_detail, date_margin};
use sea_orm::{DatabaseConnection, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait, Set};

use crate::models::admin_models::LoginPayload;

/* ------------------------------- 주문 취소 함수 ------------------------------ */
//세션을 통해서 지금까지 해당 고객이 주문한 모든 주문 불러오기
pub async fn cancel_order(
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    if let Some(_admin_id) = session.get::<uuid::Uuid>("admin_id") {
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

    //show the customer's order list
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
                completed: ActiveValue::Set(0),
                ..Default::default()
            };

            order_detail_model.update(&conn).await.unwrap();
        }
    }
    session.remove("customer_id");
    println!("세션이 만료되었습니다.");
    } else {
        (StatusCode::NOT_FOUND, "로그인해주세요.");
        println!("로그인해주세요.");
    }
}
/* -------------------------------------------------------------------------- */


/* --------------------------------- 로그인 핸들러 -------------------------------- */  

pub async fn login_handler(
    mut session: WritableSession,
    payload: Json<LoginPayload>,
) -> impl IntoResponse{
    if payload.username == "admin" && payload.pwd == "admin" {
        session.insert("admin_id", uuid::Uuid::new_v4()).unwrap();
        println!("세션에 admin_id가 저장되었습니다.");
        //redirect to admin page
        Redirect::to("/admin")
    } else {
        Redirect::to("/login")
    }
}
/* -------------------------------------------------------------------------- */



/* -------------------------------- n일차 매출 정리 함수 (가게 문 닫을 때) -------------------------------- */
pub async fn shop_closing(
    session: ReadableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {

    if let Some(_admin_id) = session.get::<uuid::Uuid>("admin_id") {
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

    } else {
        (StatusCode::NOT_FOUND, "로그인해주세요.")
    }
}

/* -------------------------------------------------------------------------- */


/* ------------------------------- 결제 완료 함수 ------------------------------ */
pub async fn payment_complete(
    admin_session: ReadableSession,
    mut customer_session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {

        if let Some(_admin_id) = admin_session.get::<uuid::Uuid>("admin_id") {
        //find session's customer_id
        let customer_id = customer_session.get::<uuid::Uuid>("customer_id").unwrap();
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
        customer_session.remove("customer_id");
        println!("세션이 만료되었습니다.");
    } else {
        (StatusCode::NOT_FOUND, "로그인해주세요.");
        println!("로그인해주세요.");
    }
}
/* -------------------------------------------------------------------------- */

