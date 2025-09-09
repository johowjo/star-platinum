use crate::Db;
use async_graphql::{Context, Object, Result};
use entity::stand::{self, Model};
use sea_orm::{DatabaseConnection, EntityTrait, SqlxMySqlConnector};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "world".to_string()
    }

    async fn stand<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Model>> {
        let db = ctx.data::<Db>()?;

        let db: DatabaseConnection =
            SqlxMySqlConnector::from_sqlx_mysql_pool(db.clone().mysql_pool);

        let stands = stand::Entity::find().all(&db).await?;

        Ok(stands)
    }
}
