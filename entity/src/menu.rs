use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub menu_id: i32,
    pub name: String,
    pub price: i32,
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