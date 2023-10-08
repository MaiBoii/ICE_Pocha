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