mod mutation;
mod query;
mod subscription;
use crate::Db;
use async_graphql::{EmptySubscription, Schema};
use mutation::Mutation;
use query::Query;

pub fn build_schema(db: Db) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .finish()
}
