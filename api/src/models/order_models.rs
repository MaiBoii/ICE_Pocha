use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateOrderModel {
    pub customer_id: Uuid,
    pub order_time: NaiveDateTime,
    pub order_items: Vec<OrderItemModel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderItemModel {
    pub menu_id: i32,
    pub quantity: i32,
}