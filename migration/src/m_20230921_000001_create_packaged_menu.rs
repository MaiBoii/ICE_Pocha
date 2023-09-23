use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230921_000001_create_packaged_menu"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PackagedMenu::Table)
                    .col(
                        ColumnDef::new(PackagedMenu::MenuId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PackagedMenu::Name).string().not_null())
                    .col(ColumnDef::new(PackagedMenu::Price).integer().not_null())
                    .col(ColumnDef::new(PackagedMenu::ProfitMargin).integer().not_null())
                    .to_owned(),
            )
            .await?;

            let insert = Query::insert()
            .into_table(PackagedMenu::Table)
            .columns([PackagedMenu::Name, PackagedMenu::Price, PackagedMenu::ProfitMargin])
            .values_panic(["탕후루".into(), 230000.into(),5000.into()])
            .values_panic(["마라샹궈".into(), 32000.into(),23000.into()])
            .values_panic(["한우 오마카세".into(), 132000.into(),100000.into()])
            .values_panic(["취두부".into(), 170000.into(),50000000.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PackagedMenu::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum PackagedMenu {
    Table,
    MenuId,
    Name,
    Price,
    ProfitMargin,
}
