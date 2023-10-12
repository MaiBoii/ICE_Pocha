use sea_orm_migration::prelude::*;

use super::m_20230921_000001_create_packaged_menu::PackagedMenu;
use super::m_20230921_000002_create_inmarket_menu::InmarketMenu;
use super::m_20230921_000003_create_order::Order;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20230921_000004_create_OrderDetails_detail"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderDetail::Table)
                    .col(
                        ColumnDef::new(OrderDetail::OrderDetailId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderDetail::OrderId).integer().not_null())
                    .col(ColumnDef::new(OrderDetail::CustomerId).string().not_null())
                    .col(ColumnDef::new(OrderDetail::PackagedMenuId).integer())
                    .col(ColumnDef::new(OrderDetail::InmarketMenuId).integer())
                    .col(ColumnDef::new(OrderDetail::Quantity).integer().not_null())
                    .col(ColumnDef::new(OrderDetail::SubTotalPrice).integer().not_null())
                    .col(ColumnDef::new(OrderDetail::TotalMargin).integer().not_null())
                    .col(ColumnDef::new(OrderDetail::Completed).boolean().not_null().default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders_detail-order_id")
                            .from(OrderDetail::Table, OrderDetail::OrderId)
                            .to(Order::Table, Order::OrderId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders_detail-packaged_id")
                            .from(OrderDetail::Table, OrderDetail::PackagedMenuId)
                            .to(PackagedMenu::Table, PackagedMenu::MenuId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders_detail-inmarket_id")
                            .from(OrderDetail::Table, OrderDetail::InmarketMenuId)
                            .to(InmarketMenu::Table, InmarketMenu::MenuId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderDetail::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum OrderDetail {
    Table,
    OrderDetailId,
    OrderId,
    CustomerId,
    PackagedMenuId,
    InmarketMenuId,
    Quantity,
    SubTotalPrice,
    TotalMargin,
    Completed,
}
