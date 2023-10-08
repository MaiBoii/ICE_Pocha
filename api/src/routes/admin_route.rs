use axum::{
    Router, 
    http::Method, 
    routing::get};
use tower_http::cors::{CorsLayer, Any};
use crate::handlers::admin_handler;

pub fn admin_routes() -> Router {

    let auth_layer = AuthLayer::new(user_store, &secret);
    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET])
    .allow_origin(Any);

    let router = Router::new()
    .route("/api/admin/login",post(admin_handler::login))
    .route("/api/admin/payed",get(admin_handler::payment_complete))
    .route("/api/admin/canceled",get(admin_handler::cancel_order))
    .route("/api/admin/shopclose",get(admin_handler::show_inmarket_menus))
    .layer(cors)
    .layer(auth_layer)
    .layer(session_layer);
    router
}

