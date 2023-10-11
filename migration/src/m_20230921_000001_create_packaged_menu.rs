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
            .values_panic(["콩불 中".into(), 16000.into(),0.into()])
            .values_panic(["콩불 大".into(), 20000.into(),0.into()])
            .values_panic(["대패숙주볶음".into(), 17000.into(),0.into()])
            .values_panic(["간장비빔국수".into(), 10000.into(),0.into()])
            .values_panic(["삼겹비빔면".into(), 10000.into(),0.into()])
            .values_panic(["오뎅탕".into(), 12000.into(),0.into()])
            .values_panic(["소시지".into(), 10000.into(),0.into()])
            .values_panic(["감자콤보".into(), 14000.into(),0.into()])
            .values_panic(["물 500ml".into(), 1000.into(),0.into()])
            .values_panic(["펩시 뚱캔".into(), 2000.into(),0.into()])
            .values_panic(["칠성 뚱캔".into(), 2000.into(),0.into()])
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
