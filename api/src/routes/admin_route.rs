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
            .route("/payed",get(admin_handler::payment_complete))
            .route("/canceled",get(admin_handler::cancel_order))
            .route("/shopclose",get(admin_handler::shop_closing))
    );
    router
}