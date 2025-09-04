mod db;
mod routes;
mod graphql;
use db::Db;
use sea_orm::{DatabaseConnection, SqlxMySqlConnector};
use std::env::var;
use routes::build_routes;
use graphql::build_schema;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port: String = var("PORT").expect("PORT is not set");
    let db = Db::init().await;

    let pool = db.clone().mysql_pool;
    let connection: DatabaseConnection = SqlxMySqlConnector::from_sqlx_mysql_pool(pool);
    const DEPTH: Option<usize> = None;
    const COMPLEXITY: Option<usize> = None;
    let schema = build_schema(connection, DEPTH, COMPLEXITY).unwrap();

    let app = build_routes(schema).await;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("GUI available at http://localhost:{port}/admin");
    axum::serve(listener, app).await.unwrap();
}
