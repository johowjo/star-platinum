use async_graphql::dynamic::{Schema, SchemaError};
use entity::{sea_orm_active_enums::*, stand};
use sea_orm::DatabaseConnection;
use seaography::{
    Builder, BuilderContext, lazy_static, register_active_enums, register_entity_modules,
};

register_entity_modules!([stand]);
register_active_enums!([
    DestructivePower,
    DevelopmentPotential,
    Persistance,
    Precision,
    Range,
    Speed
]);

lazy_static::lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn build_schema(
    db: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let builder = Builder::new(&CONTEXT, db.clone());
    let builder = register_entity_modules(builder);
    let builder = register_active_enums(builder);
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(db)
        .finish()
}
