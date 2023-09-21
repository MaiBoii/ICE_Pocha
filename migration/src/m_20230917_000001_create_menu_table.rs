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
                        ColumnDef::new(Menu::MenuId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Menu::Name).string().not_null())
                    .col(ColumnDef::new(Menu::Price).integer().not_null())
                    .col(ColumnDef::new(Menu::Togo).boolean().default(true).not_null())
                    .to_owned(),
            )
            .await?;

            let insert = Query::insert()
            .into_table(Menu::Table)
            .columns([Menu::Name, Menu::Price, Menu::Togo])
            .values_panic(["마라탕".into(), 10000.into(),false.into()])
            .values_panic(["탕후루".into(), 230000.into(),true.into()])
            .values_panic(["오뎅탕".into(), 123000.into(),false.into()])
            .values_panic(["마라샹궈".into(), 32000.into(),true.into()])
            .values_panic(["한우 오마카세".into(), 132000.into(),true.into()])
            .values_panic(["취두부".into(), 170000.into(),true.into()])
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
    MenuId,
    Name,
    Price,
    Togo,
}