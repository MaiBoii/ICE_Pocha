//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub order_id: i32,
    pub tables_id: i32,
    pub ordered_at: DateTime,
    pub total_price: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::orders_detail::Entity")]
    OrdersDetail,
}

impl Related<super::orders_detail::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrdersDetail.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
