use std::convert::Infallible;
use std::{env, net::TcpListener};

use axum::body::Body;
use axum::response::Response;
use axum::http::Request;
use axum::{Json, http::StatusCode, response::IntoResponse, Extension, extract::Query};
use axum_sessions::extractors::WritableSession;
use entity::{order, order_detail, date_margin, inmarket_menu, packaged_menu};
use sea_orm::{DatabaseConnection, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, ActiveModelTrait, Set, QueryOrder};

use crate::models::admin_models::{LoginPayload, TableParams, RevenueModel, IncompleteOrderDetail, AllIncompleteOrderDetail, UpdateOrderModel, CutomerIdParams};


/* --------------------------------- 로그인 핸들러 -------------------------------- */  

pub async fn login_handler(
    payload: Json<LoginPayload>,
) -> impl IntoResponse{
    
    dotenvy::dotenv().ok();

    let admin_id = env::var("ADMIN_ID").expect("HOST is not set in .env file");
    let admin_pw = env::var("ADMIN_PW").expect("PORT is not set in .env file");

    if payload.username == admin_id && payload.pwd == admin_pw {
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
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
            let table_id = params.table_id.as_deref().unwrap();

            let customer_id = order::Entity::find()
                //sort by order_id
                .filter(order::Column::TablesId.contains(table_id.to_string()))
                .order_by_desc(order::Column::OrderId) 
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

            let mut incomplete_order_list: Vec<IncompleteOrderDetail> = Vec::new();

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

                    if order_detail.completed == 0 && order_detail.inmarket_menu_id.is_some(){
                        incomplete_order_list.push(IncompleteOrderDetail {
                            name: inmarket_menu::Entity::find_by_id(
                                order_detail.inmarket_menu_id.unwrap_or(0)
                            )
                            .one(&conn)
                            .await
                            .unwrap()
                            .unwrap_or(inmarket_menu::Model {
                                menu_id: 0,
                                name: "메뉴가 없습니다.".to_string(),
                                price: 0,
                                profit_margin: 0,
                            })
                            .name,
                            quantity: order_detail.quantity,
                            price: order_detail.sub_total_price,
                            order_detail_id: order_detail.order_detail_id,
                        });
                    }
                }
            }

         (StatusCode::OK, Json(incomplete_order_list))
    }

/* -------------------------------------------------------------------------- */


/* ------------------------- 매장/포장 미완료 주문들 모음 ------------------------- */
pub async fn show_all_incomplete_orders(
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse{
    
    let order_detail_id = order_detail::Entity::find()
        .filter(order_detail::Column::Completed.contains("0"))
        .all(&conn)
        .await
        .unwrap()
        .into_iter()
        .map(|item| item.order_detail_id)
        .collect::<Vec<i32>>();

    let mut incomplete_order_list: Vec<AllIncompleteOrderDetail> = Vec::new();

    for detail_id in order_detail_id {
        let order_detail = order_detail::Entity::find_by_id(detail_id)
            .one(&conn)
            .await
            .unwrap()
            .unwrap();

        if order_detail.completed == 0 && order_detail.inmarket_menu_id.is_some() {
            incomplete_order_list.push(AllIncompleteOrderDetail {
                name: inmarket_menu::Entity::find_by_id(
                    order_detail.inmarket_menu_id.unwrap_or(0)
                )
                .one(&conn)
                .await
                .unwrap()
                .unwrap()
                .name,
                quantity: order_detail.quantity,
                price: order_detail.sub_total_price,
                table_id: order::Entity::find_by_id(order_detail.order_id)
                    .one(&conn)
                    .await
                    .unwrap()
                    .unwrap()
                    .tables_id,
                customer_id: order::Entity::find_by_id(order_detail.order_id)
                    .one(&conn)
                    .await
                    .unwrap()
                    .unwrap()
                    .customer_id,
            });
        } 
        else if order_detail.completed == 0 && order_detail.packaged_menu_id.is_some() {
            incomplete_order_list.push(AllIncompleteOrderDetail {
                name: packaged_menu::Entity::find_by_id(
                    order_detail.packaged_menu_id.unwrap_or(0)
                )
                .one(&conn)
                .await
                .unwrap()
                .unwrap()
                .name,
                quantity: order_detail.quantity,
                price: order_detail.sub_total_price,
                table_id: order::Entity::find_by_id(order_detail.order_id)
                    .one(&conn)
                    .await
                    .unwrap()
                    .unwrap()
                    .tables_id,
                customer_id: order::Entity::find_by_id(order_detail.order_id)
                    .one(&conn)
                    .await
                    .unwrap()
                    .unwrap()
                    .customer_id,
            });
        }
        else {
            incomplete_order_list.push(AllIncompleteOrderDetail {
                name: "메뉴가 없습니다.".to_string(),
                quantity: order_detail.quantity,
                price: order_detail.sub_total_price,
                table_id: order::Entity::find_by_id(order_detail.order_id)
                    .one(&conn)
                    .await
                    .unwrap()
                    .unwrap()
                    .tables_id,
                customer_id: order::Entity::find_by_id(order_detail.order_id)
                    .one(&conn)
                    .await
                    .unwrap()
                    .unwrap()
                    .customer_id,
            });
            
        }
    }
    (StatusCode::OK, Json(incomplete_order_list))
}
/* -------------------------------------------------------------------------- */


/* ------------------------------- 매장 주문 수정 함수 ------------------------------ */
pub async fn update_order(
    Extension(conn): Extension<DatabaseConnection>,
    Json(orders_detail): Json<UpdateOrderModel>,
) -> impl IntoResponse {

    let order_detail = order_detail::Entity::find_by_id(orders_detail.order_detail_id)
        .one(&conn)
        .await
        .unwrap()
        .unwrap();

    //update the order_detail
    let order_detail_model = order_detail::ActiveModel {
        order_detail_id: ActiveValue::Set(orders_detail.order_detail_id),
        quantity: ActiveValue::Set(orders_detail.quantity),
        sub_total_price: ActiveValue::Set(order_detail.sub_total_price * orders_detail.quantity),
        total_margin: ActiveValue::Set(order_detail.total_margin * orders_detail.quantity),
        ..Default::default()
    };

    let _ = order_detail_model.update(&conn).await;


    (StatusCode::OK, "주문이 수정되었습니다.")
}


/* -------------------------------매장 결제 완료 함수 ------------------------------ */

pub async fn inmarket_payment_complete(
    Query(params): Query<TableParams>,
    mut session: WritableSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {

            let table_id = params.table_id.as_deref().unwrap();

            //find the the last customer_id from the table_id 
            let customer_id = order::Entity::find()
                //sort by order_id
                .filter(order::Column::TablesId.contains(table_id.to_string()))
                .order_by_desc(order::Column::OrderId) 
                .one(&conn)
                .await
                .unwrap()
                .unwrap()
                .customer_id;

            println!("customer_id: {:?}", customer_id);

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
                    .filter(order_detail::Column::Completed.contains("0"))
                    .filter(order_detail::Column::InmarketMenuId.is_not_null())
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
            //Remove the payed cutomer's session
            session.remove("customer_id");

            (StatusCode::OK, "결제 완료 및 고객 세션이 만료되었습니다.")
        }
/* -------------------------------------------------------------------------- */

/* -------------------------------포장 결제 완료 함수 (진행중) ------------------------------ */
pub async fn packaged_payment_complete(
    Query(params): Query<CutomerIdParams>,
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Response<Body>, Infallible>  {

        let customer_id = params.customer_id.as_deref().unwrap();

        let order_detail_id = order_detail::Entity::find()
                .filter(order_detail::Column::Completed.contains("0"))
                .filter(order_detail::Column::PackagedMenuId.is_not_null())
                .filter(order_detail::Column::CustomerId.contains(customer_id))
                .all(&conn)
                .await
                .unwrap()
                .into_iter()
                .map(|item| item.order_detail_id)
                .collect::<Vec<i32>>();

            //update the order_detail
            for id in order_detail_id {
                let order_detail_model = order_detail::ActiveModel {
                    order_detail_id: ActiveValue::Set(id),
                    completed: ActiveValue::Set(1),
                    ..Default::default()
            };

                order_detail_model.update(&conn).await.unwrap();
            }

            let response = Response::new(Body::from("Payment complete"));
            Ok(response)
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

        (StatusCode::OK, "일자가 변경되었습니다.")
}

/* -------------------------------------------------------------------------- */

/* -------------------------------- 매출 조회 함수 -------------------------------- */
pub async fn show_revenue(
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
        let date_revenue: Vec<RevenueModel> = date_margin::Entity::find()
        .all(&conn)
        .await.unwrap()
        .into_iter()
        .map(|item| RevenueModel {
            date: item.date_margin_id,
            revenue: item.revenue,
            margin: item.profit_margin,
        }).collect();

        (StatusCode::OK, Json(date_revenue))
}
/* -------------------------------------------------------------------------- */