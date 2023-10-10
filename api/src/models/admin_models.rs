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

#[derive(Serialize, Deserialize)]
pub struct IncompleteOrderDetail {
    pub name: String,
    pub quantity: i32,
    pub price: i32,
}