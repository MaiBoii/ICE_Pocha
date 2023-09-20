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
}

#[derive(Serialize, Deserialize)]
pub struct OrdersDetailStruct {
    pub ordersdetail_id: i32,
    pub order_id: i32,
    pub menu_id: i32,
    pub quantity: i32,
    pub price: i32,
}
