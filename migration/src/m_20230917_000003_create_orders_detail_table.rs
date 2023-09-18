#![allow(unused)]
use sea_orm_migration::prelude::*;

use super::m_20230917_000001_create_menu_table::Menu;
use super::m_20230917_000002_create_order_table::Order;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230917_000003_create_orders_detail_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrdersDetail::Table)
                    .col(
                        ColumnDef::new(OrdersDetail::order_details_id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrdersDetail::order_id).integer().not_null())
                    .col(ColumnDef::new(OrdersDetail::menu_id).integer().not_null())
                    .col(ColumnDef::new(OrdersDetail::quantity).integer().not_null())
                    .col(ColumnDef::new(OrdersDetail::price).integer().not_null())
                    .col(ColumnDef::new(OrdersDetail::requests).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-OrdersDetail-menu_id")
                            .from(OrdersDetail::Table, OrdersDetail::menu_id)
                            .to(Menu::Table, Menu::menu_id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-OrdersDetail-order_id")
                            .from(OrdersDetail::Table, OrdersDetail::order_id)
                            .to(Order::Table, Order::order_id),
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
    order_details_id,
    order_id,
    menu_id,
    quantity,
    price,
    requests,
}
