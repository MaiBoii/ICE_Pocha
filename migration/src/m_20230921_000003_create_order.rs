use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230921_000003_create_order"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .col(
                        ColumnDef::new(Order::OrderId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::CustomerId).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Order::TablesId).string().not_null())
                    .col(ColumnDef::new(Order::OrderTime).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Order {
    Table,
    OrderId,
    CustomerId,
    TablesId,
    OrderTime,
}
