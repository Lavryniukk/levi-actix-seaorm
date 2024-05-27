use dotenvy::dotenv;
use sea_orm::{ Database, DatabaseConnection, DbErr };

#[derive(Clone)]
pub struct DatabaseContext {
    pub db: DatabaseConnection,
}

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Database::connect(database_url).await
}

