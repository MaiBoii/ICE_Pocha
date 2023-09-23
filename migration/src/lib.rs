use sea_orm_migration::prelude::*;

mod m_20230921_000001_create_packaged_menu;
mod m_20230921_000002_create_inmarket_menu;
mod m_20230921_000003_create_order;
mod m_20230921_000004_create_orders_detail;
mod m_20230921_000005_create_date_margin;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_20230921_000001_create_packaged_menu::Migration),
            Box::new(m_20230921_000002_create_inmarket_menu::Migration),
            Box::new(m_20230921_000003_create_order::Migration),
            Box::new(m_20230921_000004_create_orders_detail::Migration),
            Box::new(m_20230921_000005_create_date_margin::Migration),
        ]
    }
}