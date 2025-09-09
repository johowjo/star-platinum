mod graphiql;
use async_graphql::{ObjectType, Schema, SubscriptionType};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::Router;
use axum::routing::{get, post_service};
use graphiql::build_graphiql;


pub async fn build_routes<Q, M, S>(schema: Schema<Q, M, S>) -> Router 
where 
    Q: ObjectType + Sync + Send + 'static,
    M: ObjectType + Sync + Send + 'static,
    S: SubscriptionType + Sync + Send + 'static,
{

    let app = Router::new()
        .route("/graphql", get(build_graphiql).post_service(GraphQL::new(schema.clone())))
        .route_service("/graphql/ws", GraphQLSubscription::new(schema));
    Router::new()
        .nest("/api", app)
}
