use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

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
