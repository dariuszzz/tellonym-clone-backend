use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        manager.exec_stmt(schema.create_table_from_entity(entity::user::Entity)).await?;
        manager.exec_stmt(schema.create_table_from_entity(entity::question::Entity)).await?;
        manager.exec_stmt(schema.create_table_from_entity(entity::answer::Entity)).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(entity::answer::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(entity::question::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(entity::user::Entity).to_owned())
            .await
    }
}