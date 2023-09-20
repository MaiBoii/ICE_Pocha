#![allow(unused)]
use sea_orm_migration::prelude::*;

use super::m_20230917_000001_create_menu_table::Menu;
use super::m_20230917_000002_create_order_table::Order;
#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrdersDetail::Table)
                    .col(
                        ColumnDef::new(OrdersDetail::OrderDetailsId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrdersDetail::OrderId).integer().not_null())
                    .col(ColumnDef::new(OrdersDetail::MenuId).integer().not_null())
                    .col(ColumnDef::new(OrdersDetail::Quantity).integer().not_null())
                    .col(ColumnDef::new(OrdersDetail::Price).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-OrdersDetail-menu_id")
                            .from(OrdersDetail::Table, OrdersDetail::MenuId)
                            .to(Menu::Table, Menu::MenuId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-OrdersDetail-order_id")
                            .from(OrdersDetail::Table, OrdersDetail::OrderId)
                            .to(Order::Table, Order::OrderId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrdersDetail::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum OrdersDetail {
    Table,
    OrderDetailsId,
    OrderId,
    MenuId,
    Quantity,
    Price,
}
