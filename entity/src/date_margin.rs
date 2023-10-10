use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "date_margin")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub date_margin_id: i32,
    pub revenue: i32,
    pub profit_margin: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
