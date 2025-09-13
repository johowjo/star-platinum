mod mutation;
mod query;
mod subscription;
use crate::Db;
use async_graphql::Schema;
use mutation::Mutation;
use query::Query;
use subscription::Subscription;

pub fn build_schema(db: Db) -> Schema<Query, Mutation, Subscription> {
    Schema::build(Query, Mutation, Subscription)
        .data(db)
        .finish()
}
