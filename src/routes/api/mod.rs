use async_graphql::dynamic::Schema;
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::routing::{Router, get, post};

mod graphql;
mod auth;
mod panel;
use auth::*;
use graphql::graphiql::*;
use panel::*;

pub async fn build_api_routes(schema: Schema) -> Router {
    Router::new()
        .route("/admin/config", get(panel_config))
        .route("/auth/login", post(user_login))
        .route("/user/current", get(current_user))
        .route(
            "/graphql",
            get(build_graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/graphql/ws", GraphQLSubscription::new(schema))
}
