use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MenuStruct {
    pub menu_id: i32,
    pub name: String,
    pub price: i32,
}

