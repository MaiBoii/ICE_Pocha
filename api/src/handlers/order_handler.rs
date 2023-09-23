// use axum::{http::StatusCode, response::IntoResponse, extract::Query, Json, Extension};
// use chrono::Utc;
// //use entity::{order, orders_detail, menu};
// use sea_orm::{ActiveModelTrait, Set, EntityTrait, ColumnTrait, QueryFilter, DatabaseConnection};
// use serde::Deserialize;
// use uuid::Uuid;

// use crate::models::order_models::SendOrdersDetailModel;


// #[derive(Debug, Deserialize)]
// pub struct TableParams{
//     table_id: Option<String>
// }

// /* ------------------------------ orders_detail ----------------------------- */
// //OrdersDetail 정보 생성
// pub async fn send_orders_detail(
//     Query(params):Query<TableParams>, 
//     Extension(conn): Extension<DatabaseConnection>,
//     Json(orders_detail):Json<SendOrdersDetailModel>
// ) 
// -> impl IntoResponse {

// let table_id = params.table_id.as_deref().unwrap();

// //현재 tables_id와 시간 데이터를 담는다
// let order_model = order::ActiveModel {
//     tables_id: Set(table_id.to_owned()),
//     customer_id: Set(Uuid::new_v4()),
//     ordered_at: Set(Utc::now().naive_utc()),
//     ..Default::default()
// };

// order_model.insert(&conn).await.unwrap();

// //get order_model's order_id
// let order_id = order::Entity::find()
//     .filter(order::Column::TablesId.contains(table_id))
//     .one(&conn)
//     .await
//     .unwrap()
//     .unwrap()
//     .order_id;

// //추가된 order 정보를 담는다
// let orders_detail_model = orders_detail::ActiveModel {
//     order_id: Set(order_id.to_owned()),
//     menu_id: Set(orders_detail.menu_id.to_owned()),
//     quantity: Set(orders_detail.quantity.to_owned()),
//     price: Set(0.to_owned()),
//     ..Default::default()
// };
// let add_orders_detail = orders_detail_model.insert(&conn).await.unwrap();

// let menu_price = menu::Entity::find_by_id(add_orders_detail.menu_id)
//     .one(&conn)
//     .await
//     .unwrap()
//     .unwrap()
//     .price;

// let update_total_price = orders_detail::ActiveModel {
//     order_details_id: Set(add_orders_detail.order_details_id),
//     price: Set(menu_price * add_orders_detail.quantity),
//     ..Default::default()
// };

// update_total_price.update(&conn).await.unwrap();

// println!("{} {}", menu_price, add_orders_detail.quantity);

// (StatusCode::ACCEPTED, "주문이 완료되었습니다.")
// }
// /* -------------------------------------------------------------------------- */


