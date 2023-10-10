use std::env;

use axum::{Json, http::StatusCode, response::IntoResponse, Extension, extract::Query};
use axum_sessions::extractors::{WritableSession, ReadableSession};
use entity::{order, order_detail, date_margin, inmarket_menu};
use sea_orm::{DatabaseConnection, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait, Set};

use crate::models::admin_models::{LoginPayload, TableParams, RevenueModel, IncompleteOrderDetail};

/* --------------------------------- 로그인 핸들러 -------------------------------- */  

pub async fn login_handler(
    mut session: WritableSession,
    payload: Json<LoginPayload>,
) -> impl IntoResponse{
    
    dotenvy::dotenv().ok();

    let admin_id = env::var("ADMIN_ID").expect("HOST is not set in .env file");
    let admin_pw = env::var("ADMIN_PW").expect("PORT is not set in .env file");
    
    println!("session: {:?}", session);
    if payload.username == admin_id && payload.pwd == admin_pw {
        session.insert("admin_id", true)
                .expect("Failed to insert admin_id into session");

        println!("세션에 admin_id가 저장되었습니다.");
        println!("{}, {}",payload.username, payload.pwd);
        println!("{}",session.get::<bool>("admin_id").unwrap());
        println!("{:?}", session); 

        (StatusCode::OK, "로그인 성공")
        
    } else {
        println!("세션에 admin_id가 저장되지 않았습니다.");
        println!("{}, {}",payload.username, payload.pwd);
        (StatusCode::NOT_FOUND, "로그인 실패")

    }
}
/* -------------------------------------------------------------------------- */


/* ------------------------- 테이블 번호 별로 미완료 주문들 보내주기 ------------------------ */

pub async fn show_incomplete_orders(
    Query(params): Query<TableParams>,
    session: ReadableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse{
    if session.get::<bool>("admin_id").unwrap_or(false){
        println!("{}",session.get::<bool>("admin_id").unwrap());

        println!("{:?}", session);
        let table_id = params.table_id.as_deref().unwrap();

        println!("table_id: {}", table_id);
    
        //테이블 아이디로 가장 최근 주문한 고객 아이디 찾기
        let customer_id = order::Entity::find()
            .filter(order::Column::TablesId.contains(table_id.to_string()))
            .one(&conn)
            .await
            .unwrap()
            .unwrap()
            .customer_id;

        //println!("customer_id: {}", customer_id);
    
        //고객 아이디로 주문 아이디 전부 찾기
        let order_id = order::Entity::find()
            .filter(order::Column::CustomerId.contains(customer_id.to_string()))
            .all(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|item| item.order_id)
            .collect::<Vec<i32>>();
    
        //show the customer's order list
        let mut incomplete_order_list: Vec<IncompleteOrderDetail>  = Vec::new();

        for id in order_id {
            let order_detail_id = order_detail::Entity::find()
                .filter(order_detail::Column::OrderId.contains(id.to_string()))
                .all(&conn)
                .await
                .unwrap()
                .into_iter()
                .map(|item| item.order_detail_id)
                .collect::<Vec<i32>>();
            println!("order_detail_id: {:?}", order_detail_id);

            for detail_id in order_detail_id {
                let order_detail = order_detail::Entity::find_by_id(detail_id)
                    .one(&conn)
                    .await
                    .unwrap()
                    .unwrap();
                println!("order_detail: {:?}", order_detail);

                if order_detail.completed == 0 {
                    incomplete_order_list.push(IncompleteOrderDetail {
                        name: inmarket_menu::Entity::find_by_id(
                            order_detail
                            .inmarket_menu_id
                            .unwrap_or(0)
                        )
                            .one(&conn)
                            .await
                            .unwrap()
                            //error handling for unwrap
                            .unwrap_or(inmarket_menu::Model {
                                menu_id: 0,
                                name: "메뉴가 없습니다.".to_string(),
                                price: 0,
                                profit_margin: 0,
                            })
                            .name,
                        quantity: order_detail.quantity,
                        price: order_detail.sub_total_price,
                    });
                }
            }
        }
        //println!("incomplete_order_list: {:?}", incomplete_order_list);

        (StatusCode::OK, Json(incomplete_order_list))

        }
        else {
            println!("{}",session.get::<bool>("admin_id").unwrap_or(false));
            println!("{:?}", session);
            (StatusCode::NON_AUTHORITATIVE_INFORMATION, Json(vec![IncompleteOrderDetail {
                name: "로그인해주세요.".to_string(),
                quantity: 0,
                price: 0,
            }]))
        }

}
/* -------------------------------------------------------------------------- */

/* ------------------------------- 주문 취소 함수 ------------------------------ */

pub async fn cancel_order(
    Query(params): Query<TableParams>,
    session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    
    if session.get::<bool>("admin_id").unwrap_or(false){
    //find session's customer_id
    let table_id = params.table_id.as_deref().unwrap();


    //find a the most recent customer_id with table_id
    let customer_id = order::Entity::find()
        .filter(order::Column::TablesId.contains(table_id.to_string()))
        .one(&conn)
        .await
        .unwrap()
        .unwrap()
        .customer_id;

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
        (StatusCode::OK, "취소 완료 되었습니다.")
    } else {
        (StatusCode::NOT_FOUND, "로그인해주세요.")
    }
}
/* -------------------------------------------------------------------------- */


/* ------------------------------- 결제 완료 함수 ------------------------------ */
pub async fn payment_complete(
    Query(params): Query<TableParams>,
    session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {

    if session.get::<bool>("admin_id").unwrap_or(false){

            let table_id = params.table_id.as_deref().unwrap();
            let customer_id = order::Entity::find()
                .filter(order::Column::TablesId.contains(table_id.to_string()))
                .one(&conn)
                .await
                .unwrap()
                .unwrap()
                .customer_id;

            let order_id = order::Entity::find()
                .filter(order::Column::CustomerId.contains(customer_id.to_string()))
                .all(&conn)
                .await
                .unwrap()
                .into_iter()
                .map(|item| item.order_id)
                .collect::<Vec<i32>>();

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
            (StatusCode::ACCEPTED, "결제 완료 및 고객 세션이 만료되었습니다.")
            } else {
                (StatusCode::NOT_FOUND, "로그인해주세요.")
            }
        }
/* -------------------------------------------------------------------------- */


/* -------------------------------- n일차 매출 정리 함수 (가게 문 닫을 때) -------------------------------- */
pub async fn shop_closing(
    session: ReadableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {

    if session.get::<bool>("admin_id").unwrap_or(false){
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

        let mut _total_revenue = 0;
        let mut _total_margin = 0;
        for id in order_detail_id {
            let order_detail = order_detail::Entity::find_by_id(id)
                .one(&conn)
                .await
                .unwrap()
                .unwrap();
            _total_revenue = order_detail.sub_total_price;
            _total_margin += order_detail.total_margin;
        }

        let todays_margin = date_margin::ActiveModel {
            revenue: Set(_total_revenue.to_owned()),
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

/* -------------------------------- 매출 조회 함수 -------------------------------- */
pub async fn show_revenue(
    admin_session: ReadableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    if let Some(_admin_id) = admin_session.get::<uuid::Uuid>("admin_id") {

        let date_revenue: Vec<RevenueModel> = date_margin::Entity::find()
        .all(&conn)
        .await.unwrap()
        .into_iter()
        .map(|item| RevenueModel {
            date: item.date_margin_id,
            revenue: item.revenue,
            margin: item.profit_margin,
        }).collect();

        (StatusCode::ACCEPTED, Json(date_revenue))
    } else {
        (StatusCode::NOT_FOUND, Json(vec![RevenueModel {
            date: 0,
            revenue: 0,
            margin: 0,
        }]))
    }
}
/* -------------------------------------------------------------------------- */