use axum::routing::{get, post};
use axum::{Router, http::Method};
use tower_http::cors::{CorsLayer, Any};
use crate::handlers::order_handler;

pub fn order_routes() -> Router {

    // let cors = CorsLayer::new()
    // .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    // .allow_origin(Any);

    let router = Router::new()
    //.route("/api/order/:uuid/update",put(order_handler::update_user_put))
    //.route("/api/order/:uuid/delete",delete(order_handler::delete_user_delete))
    .route("/api/inmarket/order",post(order_handler::order_inmarket_menus))
    .route("/api/packaged/order",post(order_handler::order_packaged_menus))
    .route("/api/inmarket/payed", post(order_handler::payment_complete))
    .route("/api/packaged/payed", post(order_handler::payment_complete))
    .route("/api/shopclose", get(order_handler::shop_closing));
    router
}