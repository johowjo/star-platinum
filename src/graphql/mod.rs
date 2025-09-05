use async_graphql::{
    Name, Value,
    dynamic::{indexmap::IndexMap, *},
};
use entity::{
    sea_orm_active_enums::*,
    stand::{self, Model},
};
use sea_orm::{DatabaseConnection, EntityTrait};
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

fn stand_enum_to_value<T: ToString>(opt: Option<T>) -> Value {
    match opt {
        Some(v) => Value::from(v.to_string()),
        None => Value::Null,
    }
}

fn model_to_value(s: Model) -> Value {
    let mut map = IndexMap::new();
    map.insert(Name::new("id"), Value::from(s.id));
    map.insert(Name::new("name"), Value::from(s.name));
    map.insert(Name::new("owner"), Value::from(s.owner));
    map.insert(
        Name::new("destructive_power"),
        stand_enum_to_value(s.destructive_power),
    );
    map.insert(Name::new("speed"), stand_enum_to_value(s.speed));
    map.insert(Name::new("persistance"), stand_enum_to_value(s.persistance));
    map.insert(Name::new("precision"), stand_enum_to_value(s.precision));
    map.insert(
        Name::new("development_potential"),
        stand_enum_to_value(s.development_potential),
    );
    Value::Object(map)
}

fn custom_field() -> Field {
    Field::new("stands", TypeRef::named_nn_list("Stand"), |ctx| {
        FieldFuture::new(async move {
            let db = ctx.data::<DatabaseConnection>()?;
            let items = stand::Entity::find().all(db).await?;

            // Convert models into GraphQL Values
            let values: Vec<Value> = items.into_iter().map(model_to_value).into_iter().collect();

            Ok(Some(Value::List(values)))
        })
    })
}

pub fn build_schema(
    db: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let builder = Builder::new(&CONTEXT, db.clone());
    let builder = register_entity_modules(builder);
    let mut builder = register_active_enums(builder);
    builder.query = builder.query.field(custom_field());
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(db)
        .finish()
}
