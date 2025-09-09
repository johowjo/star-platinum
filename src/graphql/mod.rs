pub mod mutation;
pub mod query;
pub mod subscription;
use crate::Db;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use query::Query;

pub fn build_schema(db: Db) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
