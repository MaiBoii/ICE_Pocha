use axum::routing::post;
use axum::Router;
use crate::handlers::order_handler;

pub fn order_routes() -> Router {

    let router = Router::new()
    .route("/inmarket/order",post(order_handler::order_inmarket_menus))
    .route("/packaged/order",post(order_handler::order_packaged_menus));
    router
}