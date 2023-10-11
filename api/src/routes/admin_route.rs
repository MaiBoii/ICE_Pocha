use axum::{
    Router, 
    routing::{get, post}};
    
use crate::handlers::admin_handler;

pub fn admin_routes() -> Router {
    
    let router = Router::new()
    .route("/api/login",post(admin_handler::login_handler))
    .nest(
        "/api/admin",
        Router::new()
            .route("/show",get(admin_handler::show_incomplete_orders))
            .route("/show_all",get(admin_handler::show_all_incomplete_orders))
            .route("/show/update",post(admin_handler::update_order))
            .route("/inmarket/payment",get(admin_handler::inmarket_payment_complete))
            .route("/packaged/payment",get(admin_handler::packaged_payment_complete))
            .route("/revenue",get(admin_handler::show_revenue))
            .route("/shopclose",get(admin_handler::shop_closing))
    );
    router
}