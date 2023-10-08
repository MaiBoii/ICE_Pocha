use std::env;
use axum::{Json, http::{StatusCode, Response}, response::IntoResponse, Extension, body::Body};
use axum_sessions::{extractors::{WritableSession, ReadableSession}};
use entity::{order, order_detail};
use sea_orm::{DatabaseConnection, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};
use std::sync::Arc;
use tower::BoxError;

use crate::models::admin_models::LoginPayload;

//use crate::models::admin_models::{LoginPayload, LoginResponse};

// JSON 요청 본문에 대한 구조체 정의
#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    id: String,
    password: String,
}

// JSON 응답 본문에 대한 구조체 정의
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

/* ------------------------------- 주문 취소 함수 ------------------------------ */
//세션을 통해서 지금까지 해당 고객이 주문한 모든 주문 불러오기
pub async fn cancel_order(
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
}
/* -------------------------------------------------------------------------- */


/* --------------------------------- 로그인 핸들러 -------------------------------- */  

// /login 핸들러
pub async fn login_handler(
    payload: Json<LoginPayload>,
    mut session: WritableSession,
) -> Result<Body> {
    // 사용자가 제출한 id와 password 가져오기
    let user_id = &payload.id;
    let user_password = &payload.password;

    if user_id == "valid_username" && user_password == "valid_password" {
        session.insert("signed_in", true).expect("Could not sign in.");
        // Redirect to /admin on successful login.
        Response::redirect("/admin")
    } else {
        // Redirect back to /login on failed login.
        Response::redirect("/login")
    }
}

pub async fn admin_handler(session: ReadableSession) -> Response<Body> {
    if session.get::<bool>("signed_in").unwrap_or(false) {
        Response::new(Body::from("Welcome to the admin panel"))
    } else {
        Response::new(Body::from("Access denied. Please log in."))
    }
}
/* -------------------------------------------------------------------------- */



/* -------------------------------- n일차 매출 정리 함수 (가게 문 닫을 때) -------------------------------- */
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

