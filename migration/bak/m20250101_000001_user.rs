use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(User::Table)
            .col(date_time(User::CreatedAt).default(Expr::current_timestamp()))
            .col(date_time(User::UpdatedAt).default(Expr::current_timestamp()))
            .col(big_integer(User::Id).auto_increment().primary_key().take())
            .col(uuid(User::Pid))
            .col(string_uniq(User::Email))
            .col(string(User::Password))
            .col(string(User::ApiKey).unique_key())
            .col(string(User::Name))
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    CreatedAt,
    UpdatedAt,
    Id,
    Pid,
    Email,
    Name,
    Password,
    ApiKey,
}
