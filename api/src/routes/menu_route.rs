// use axum::{Router, http::Method, routing::get};
// use tower_http::cors::{CorsLayer, Any};
// use crate::handlers::menu_handler;


// pub fn menu_routes() -> Router {

//     let cors = CorsLayer::new()
//     .allow_methods([Method::POST, Method::GET])
//     .allow_origin(Any);

//     let router = Router::new()
//     .route("/api/menu",get(menu_handler::show_menus))
//     .layer(cors);
//     router
// }