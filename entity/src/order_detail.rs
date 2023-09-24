use sea_orm::{entity::prelude::*, prelude::async_trait::async_trait, DeriveEntityModel, RelationDef, DbErr, ConnectionTrait};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "order_detail")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub order_detail_id: i32,
    pub order_id: i32,
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
impl ActiveModelBehavior for ActiveModel {

    // async fn after_save<C>(mut model: Model, db: &C, insert: bool) -> Result<Model, DbErr>
    // where
    //     C: ConnectionTrait
    // {
    //     // find menu price
    //     let menu_price = match model.inmarket_menu_id {
    //         Some(inmarket_menu_id) => {
    //             let inmarket_menu = super::inmarket_menu::Entity::find_by_id(inmarket_menu_id)
    //                 .one(db)
    //                 .await
    //                 .unwrap();
    //             inmarket_menu.unwrap().price
    //         }
    //         None => {
    //             let inmarket_menu =
    //                 super::inmarket_menu::Entity::find_by_id(model.inmarket_menu_id.unwrap())
    //                     .one(db)
    //                     .await
    //                     .unwrap();
    //             inmarket_menu.unwrap().price
    //         }
    //     };
        
    //     let total_price = menu_price * model.quantity;
    //     // 모델의 sub_total_price 필드 업데이트
    //     model.sub_total_price = total_price;
    //     println!("------------=======-=-== sub_total_price: {}", model.sub_total_price);

    //     Ok(model) // 모델 업데이트 후 반환
    // }
}
