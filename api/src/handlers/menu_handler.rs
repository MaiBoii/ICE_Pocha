use axum::{Json, http::StatusCode, response::IntoResponse, Extension};
use entity::{packaged_menu, inmarket_menu};
use sea_orm::{EntityTrait, DatabaseConnection};

use crate::models::menu_models::MenuStruct;

/* --------------------------------- 포장 메뉴 보이기 --------------------------------- */
pub async fn show_packaged_menus(
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let menus: Vec<MenuStruct> = packaged_menu::Entity::find().all(&conn).await.unwrap().into_iter().map(|item| MenuStruct {
        menu_id: item.menu_id,
        name: item.name,
        price: item.price,
    }).collect();
    
    (StatusCode::ACCEPTED,Json(menus))
}
/* -------------------------------------------------------------------------- */

/* --------------------------------- 매장 메뉴 보이기 --------------------------------- */
pub async fn show_inmarket_menus(
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let menus: Vec<MenuStruct> = inmarket_menu::Entity::find().all(&conn).await.unwrap().into_iter().map(|item| MenuStruct {
        menu_id: item.menu_id,
        name: item.name,
        price: item.price,
    }).collect();
    
    (StatusCode::ACCEPTED,Json(menus))
}
/* -------------------------------------------------------------------------- */