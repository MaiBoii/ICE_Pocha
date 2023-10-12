use sea_orm::{entity::prelude::*, prelude::async_trait::async_trait, DeriveEntityModel, RelationDef};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "order_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub order_detail_id: i32,
    pub order_id: i32,
    pub customer_id: String,
    pub packaged_menu_id: Option<i32>,
    pub inmarket_menu_id: Option<i32>,
    pub quantity: i32,
    pub sub_total_price: i32,
    pub total_margin: i32,
    pub completed: i8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::inmarket_menu::Entity",
        from = "Column::InmarketMenuId",
        to = "super::inmarket_menu::Column::MenuId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    InmarketMenu,
    #[sea_orm(
        belongs_to = "super::order::Entity",
        from = "Column::OrderId",
        to = "super::order::Column::OrderId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Order,
    #[sea_orm(
        belongs_to = "super::packaged_menu::Entity",
        from = "Column::PackagedMenuId",
        to = "super::packaged_menu::Column::MenuId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PackagedMenu,
}

impl Related<super::inmarket_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InmarketMenu.def()
    }
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::packaged_menu::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PackagedMenu.def()
    }
}
#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
