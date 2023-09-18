#![allow(unused)]
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230917_000001_create_menu_table"
    }
}

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
            .await
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