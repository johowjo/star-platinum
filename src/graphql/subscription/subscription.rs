use async_graphql::Subscription;
use entity::stand::Model;
use futures_util::Stream;

use super::subscriptor::Subscriptor;

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn subscribe_stand(&self) -> impl Stream<Item = Model> {
        Subscriptor::<Model>::subscribe()
    }
}
