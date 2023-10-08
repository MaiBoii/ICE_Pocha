use axum::{Router, routing::get};
use crate::handlers::menu_handler;


pub fn menu_routes() -> Router {

    let router = Router::new()
    .route("/packaged/menu",get(menu_handler::show_packaged_menus))
    .route("/inmarket/menu",get(menu_handler::show_inmarket_menus));
    router
}

