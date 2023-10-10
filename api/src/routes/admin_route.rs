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
            .route("/show/payment",get(admin_handler::payment_complete))
            .route("/show/cancel",post(admin_handler::cancel_order))
            .route("/revenue",get(admin_handler::show_revenue))
            .route("/shopclose",get(admin_handler::shop_closing))
    );
    router
}