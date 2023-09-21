use axum::{Json, http::StatusCode, response::IntoResponse, Extension};
use entity::menu;
use sea_orm::{EntityTrait, DatabaseConnection};

use crate::models::menu_models::MenuStruct;

/* --------------------------------- 메뉴 보이기 --------------------------------- */
pub async fn show_menus(
    Extension(conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let menus: Vec<MenuStruct> = menu::Entity::find().all(&conn).await.unwrap().into_iter().map(|item| MenuStruct {
        menu_id: item.menu_id,
        name: item.name,
        price: item.price,
        togo: item.togo,
    }).collect();
    
    (StatusCode::ACCEPTED,Json(menus))
}
/* -------------------------------------------------------------------------- */

