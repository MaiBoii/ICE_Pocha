use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct MenuStruct {
    pub menu_id: i32,
    pub name: String,
    pub price: i32,
    pub togo: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateOrderModel {
    pub table_id: String,
    pub ordered_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct SendOrdersDetailModel {
    pub menu_id: i32,
    pub quantity: i32,
}
