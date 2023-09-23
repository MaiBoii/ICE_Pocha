use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateOrderModel {
    pub order_items: Vec<OrderItemModel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderItemModel {
    pub menu_id: i32,
    pub quantity: i32,
}