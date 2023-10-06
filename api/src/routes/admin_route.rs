use axum::{Router, http::Method, routing::get};
use tower_http::cors::{CorsLayer, Any};
use crate::handlers::admin_handler;

pub fn menu_routes() -> Router {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET])
    .allow_origin(Any);

    let router = Router::new()
    .route("/api/admin/login",get(admin_handler::show_inmarket_menus))
    .route("/api/admin/payed",get(admin_handler::show_packaged_menus))
    .route("/api/admin/canceled",get(admin_handler::cancel_order))
    .route("/api/admin/shopclose",get(admin_handler::show_inmarket_menus))
    .layer(cors);
    router
}

