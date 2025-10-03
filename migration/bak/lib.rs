#![allow(
    elided_lifetimes_in_paths,
    clippy::wildcard_imports,
    clippy::enum_variant_names
)]
pub use sea_orm_migration::prelude::*;

mod m20250101_000001_user;
mod m20250101_000002_seed_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_user::Migration),
            Box::new(m20250101_000002_seed_users::Migration),
        ]
    }
}
