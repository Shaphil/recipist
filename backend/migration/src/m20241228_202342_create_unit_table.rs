use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Unit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Unit::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Unit::Name).string().not_null())
                    .col(
                        ColumnDef::new(Unit::UnitType)
                            .char()
                            .char_len(32)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Unit::Multiplier).float().null())
                    .col(ColumnDef::new(Unit::Symbol).char().char_len(16).not_null())
                    .col(ColumnDef::new(Unit::IsDefault).boolean().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Unit::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Unit {
    Table,
    Id,
    Name,
    UnitType,
    Multiplier,
    Symbol,
    IsDefault,
}

// #[derive(DeriveIden, EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "unit_types")]
// enum UnitTypes {
//     #[sea_orm(string_value = "Solid")]
//     Solid,
//     #[sea_orm(string_value = "Liquid")]
//     Liquid,
//     #[sea_orm(string_value = "SolidAndLiquid")]
//     SolidAndLiquid,
// }
