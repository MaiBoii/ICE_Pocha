// use axum::routing::get;
// use axum::{Router, http::Method};
// use tower_http::cors::{CorsLayer, Any};
// use crate::handlers::order_handler;

// pub fn order_routes() -> Router {

//     let cors = CorsLayer::new()
//     .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
//     .allow_origin(Any);

//     let router = Router::new()
//     //.route("/api/order/:uuid/update",put(order_handler::update_user_put))
//     //.route("/api/order/:uuid/delete",delete(order_handler::delete_user_delete))
//     .route("/api/order/send",get(order_handler::send_orders_detail))
//     .layer(cors);
//     router
// }