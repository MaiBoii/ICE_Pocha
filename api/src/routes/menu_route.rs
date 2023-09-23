use axum::{Router, http::Method, routing::get};
use tower_http::cors::{CorsLayer, Any};
use crate::handlers::menu_handler;


pub fn menu_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET])
    .allow_origin(Any);

    let router = Router::new()
    .route("/api/packaged/menu",get(menu_handler::show_packaged_menus))
    .route("/api/inmarket/menu",get(menu_handler::show_inmarket_menus))
    .layer(cors);
    router
}