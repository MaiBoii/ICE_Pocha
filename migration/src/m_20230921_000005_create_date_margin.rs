use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230921_000005_create_date_margin"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DateMargin::Table)
                    .col(
                        ColumnDef::new(DateMargin::DateMarginId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DateMargin::Revenue).integer().not_null())
                    .col(ColumnDef::new(DateMargin::ProfitMargin).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DateMargin::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum DateMargin {
    Table,
    DateMarginId,
    Revenue,
    ProfitMargin,
}
