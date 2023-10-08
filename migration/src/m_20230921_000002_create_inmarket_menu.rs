use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230921_000002_create_inmarket_menu"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(InmarketMenu::Table)
                    .col(
                        ColumnDef::new(InmarketMenu::MenuId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(InmarketMenu::Name).string().not_null())
                    .col(ColumnDef::new(InmarketMenu::Price).integer().not_null())
                    .col(ColumnDef::new(InmarketMenu::ProfitMargin).integer().not_null())
                    .to_owned(),
            )
            .await?;

            let insert = Query::insert()
            .into_table(InmarketMenu::Table)
            .columns([InmarketMenu::Name, InmarketMenu::Price, InmarketMenu::ProfitMargin])
            .values_panic(["대패숙주볶음".into(), 17000.into(),3000.into()])
            .values_panic(["삼겹비빔면".into(), 10000.into(),5000.into()])
            .values_panic(["오뎅창".into(), 14000.into(),30000.into()])
            .values_panic(["간장비빔국수".into(), 10000.into(),23000.into()])
            .values_panic(["콩불_소".into(), 15000.into(),100000.into()])
            .values_panic(["콩불_중".into(), 21000.into(),100000.into()])
            .values_panic(["감튀+해쉬브라운+버팔로윙".into(), 14000.into(),100000.into()])
            .values_panic(["소세지".into(), 10000.into(),50000000.into()])
            .values_panic(["펩시 뚱캔".into(), 2000.into(),50000000.into()])
            .values_panic(["칠성 뚱캔".into(), 2000.into(),50000000.into()])
            .values_panic(["물 500ml".into(), 1000.into(),50000000.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InmarketMenu::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum InmarketMenu {
    Table,
    MenuId,
    Name,
    Price,
    ProfitMargin,
}
