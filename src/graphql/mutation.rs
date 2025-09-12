use crate::Db;
use async_graphql::{Context, Object, Result};
use entity::stand::{Model, str_to_optional_stand_stat};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_stand<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
        owner: String,
        destructive_power: String,
        speed: String,
        range: String,
        persistance: String,
        precision: String,
        development_potential: String,
    ) -> Result<Model> {
        let db = ctx.data::<Db>()?;

        let result = sqlx::query!(
            "INSERT INTO Stand (name, owner, destructive_power, speed, `range`, persistance, `precision`, development_potential) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            name,
            owner,
            destructive_power,
            speed,
            range,
            persistance,
            precision,
            development_potential,
        )
        .execute(&db.mysql_pool)
        .await?;

        Ok(Model {
            id: result.last_insert_id() as i32,
            name,
            owner,
            destructive_power: str_to_optional_stand_stat(destructive_power.as_str()),
            speed: str_to_optional_stand_stat(speed.as_str()),
            range: str_to_optional_stand_stat(range.as_str()),
            persistance: str_to_optional_stand_stat(persistance.as_str()),
            precision: str_to_optional_stand_stat(precision.as_str()),
            development_potential: str_to_optional_stand_stat(development_potential.as_str()),
        })
    }
}
