use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

static STAND_PARAMETER_VALUE: &str = "ENUM('A', 'B', 'C', 'D', 'E')";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Stand::Table)
                    .if_not_exists()
                    .col(pk_auto(Stand::Id))
                    .col(string(Stand::Name))
                    .col(string(Stand::Owner))
                    .col(ColumnDef::new(Stand::DestructivePower).custom(Alias::new(STAND_PARAMETER_VALUE)))
                    .col(ColumnDef::new(Stand::Speed).custom(Alias::new(STAND_PARAMETER_VALUE)))
                    .col(ColumnDef::new(Stand::Range).custom(Alias::new(STAND_PARAMETER_VALUE)))
                    .col(ColumnDef::new(Stand::Persistance).custom(Alias::new(STAND_PARAMETER_VALUE)))
                    .col(ColumnDef::new(Stand::Precision).custom(Alias::new(STAND_PARAMETER_VALUE)))
                    .col(ColumnDef::new(Stand::DevelopmentPotential).custom(Alias::new(STAND_PARAMETER_VALUE)))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Stand::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Stand {
    Table,
    Id,
    Name,
    Owner,
    DestructivePower,
    Speed,
    Range,
    Persistance,
    Precision,
    DevelopmentPotential,
}
