use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct MenuStruct {
    pub menu_id: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize)]
pub struct OrderStruct {
    pub order_id: i32,
    pub table_id: String,
    pub ordered_at: NaiveDateTime,
    pub total_amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct OrdersDetailStruct {
    ordersdetail_id: i32,
    order_id: i32,
    menu_id: i32,
    quantity: i32,
    price: i32,
    requests: String,
}
