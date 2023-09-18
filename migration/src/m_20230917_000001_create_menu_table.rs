use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Menu::Table)
                    .col(
                        ColumnDef::new(Menu::menu_id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Menu::name).string().not_null())
                    .col(ColumnDef::new(Menu::price).integer().not_null())
                    .to_owned(),
            )
            .await?;

            let insert = Query::insert()
            .into_table(Menu::Table)
            .columns([Menu::name, Menu::price])
            .values_panic(["마라탕".into(), 10000.into()])
            .values_panic(["탕후루".into(), 230000.into()])
            .values_panic(["양장피".into(), 123000.into()])
            .values_panic(["마라샹궈".into(), 32000.into()])
            .values_panic(["한우 오마카세".into(), 132000.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    // 오류발생시 어떻게 롤백할지 정의: Drop the Menu table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Menu {
    Table,
    menu_id,
    name,
    price,
}