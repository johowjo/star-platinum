mod db;
mod graphql;
mod routes;
use db::Db;
use graphql::build_schema;
use routes::build_routes;
use std::env::var;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port: String = var("PORT").expect("PORT is not set");
    let db = Db::init().await;

    let schema = build_schema(db.clone());

    let app = build_routes(schema).await;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("GraphQL available at http://localhost:{port}/api/graphql");

    axum::serve(listener, app).await.unwrap();
}
