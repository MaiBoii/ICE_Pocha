use axum::{
    Router, 
    routing::{get, post}};
    
use crate::handlers::admin_handler;

pub fn admin_routes() -> Router {

    let router = Router::new()
    .route("/login",post(admin_handler::login_handler))
    .nest(
        "/admin",
        Router::new()
            .route("api/show",get(admin_handler::show_incomplete_orders))
            .route("api/show/payment",get(admin_handler::payment_complete))
            .route("api/show/cancel",post(admin_handler::cancel_order))
            .route("api/revenue",get(admin_handler::show_revenue))
            .route("api/shopclose",get(admin_handler::shop_closing))
    );
    router
}