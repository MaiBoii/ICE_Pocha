use axum::{Router, routing::get};
use crate::handlers::menu_handler;


pub fn menu_routes() -> Router {

    let router = Router::new()
    .route("/",get(home))
    .route("/api/packaged/menu",get(menu_handler::show_packaged_menus))
    .route("/api/inmarket/menu",get(menu_handler::show_inmarket_menus));
    router
}

//handler to show home page in flutter web app
pub async fn home() -> &'static str {
    "Hello, World!"
} 
