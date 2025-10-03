use super::m20250101_000001_user::User;
use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(User::Table)
            .columns([
                User::Pid,
                User::Email,
                User::Password,
                User::ApiKey,
                User::Name,
            ])
            .values_panic([
                Uuid::new_v4().into(),
                "admin@sea-ql.org".into(),
                hash_password("demo@sea-ql.org")?.into(),
                format!("lo-{}", Uuid::new_v4()).into(),
                "Admin".into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(User::Table)
            .columns([
                User::Pid,
                User::Email,
                User::Password,
                User::ApiKey,
                User::Name,
            ])
            .values_panic([
                Uuid::new_v4().into(),
                "manager@sea-ql.org".into(),
                hash_password("demo@sea-ql.org")?.into(),
                format!("lo-{}", Uuid::new_v4()).into(),
                "Manager".into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(User::Table)
            .columns([
                User::Pid,
                User::Email,
                User::Password,
                User::ApiKey,
                User::Name,
            ])
            .values_panic([
                Uuid::new_v4().into(),
                "public@sea-ql.org".into(),
                hash_password("demo@sea-ql.org")?.into(),
                format!("lo-{}", Uuid::new_v4()).into(),
                "Public".into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

fn hash_password(s: &str) -> Result<String, DbErr> {
    loco_rs::hash::hash_password(s).map_err(|e| DbErr::Custom(e.to_string()))
}
