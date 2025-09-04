mod api;
use async_graphql::dynamic::Schema;
use axum::Router;
use axum::routing::get_service;
use dotenv;
use std::env::var;
use tower_http::services::{ServeDir, ServeFile};

pub async fn build_routes(schema: Schema) -> Router {
    dotenv::dotenv().ok();
    let sea_orm_pro_assets = var("SEA_ORM_PRO_ASSETS_PATH").unwrap();

    let panel = ServeDir::new(format!("{sea_orm_pro_assets}/admin")).fallback(ServeFile::new(
        format!("{sea_orm_pro_assets}admin/index.html"),
    ));

    let app = api::build_api_routes(schema.clone()).await;
    Router::new()
        .nest("/api", app)
        .nest_service("/admin", get_service(panel))
}
