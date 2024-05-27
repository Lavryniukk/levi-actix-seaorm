use sea_orm_migration::{ prelude::*, sea_orm::{ DeriveActiveEnum, EnumIter } };

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        use sea_orm::{ Schema, DbBackend };
        let schema = Schema::new(DbBackend::Postgres);
        manager.create_type(schema.create_enum_from_active_enum::<Role>()).await?;

        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(User::Name).string().not_null())
                .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                .col(ColumnDef::new(User::Role).string().not_null())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::CreatedAt).timestamp().not_null().default("now()"))
                .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null().default("now()"))
                .col(ColumnDef::new(User::LastSignedInAt).timestamp().not_null().default("now()"))
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Role,
    Password,
    CreatedAt,
    UpdatedAt,
    LastSignedInAt,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "Role")]
enum Role {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "User")]
    User,
}
