use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}

// Serialize response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct TableId {
    pub table_id: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct IncompleteOrderDetail {
    pub name: String,
    pub quantity: i32,
    pub price: i32,
    pub order_detail_id: i32,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AllIncompleteOrderDetail {
    pub name: String,
    pub quantity: i32,
    pub price: i32,
    pub table_id: String,
    pub customer_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TableParams{
    pub table_id: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct CutomerIdParams{
    pub customer_id: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct RevenueModel {
    pub date: i32,
    pub revenue: i32,
    pub margin: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateOrderModel {
    pub order_detail_id: i32,
    pub quantity: i32,
}