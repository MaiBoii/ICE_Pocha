pub use sea_orm_migration::prelude::*;

mod m_20230917_000001_create_menu_table;
mod m_20230917_000002_create_order_table;
mod m_20230917_000003_create_orders_detail_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_20230917_000001_create_menu_table::Migration),
            Box::new(m_20230917_000002_create_order_table::Migration),
            Box::new(m_20230917_000003_create_orders_detail_table::Migration),
        ]
    }
}