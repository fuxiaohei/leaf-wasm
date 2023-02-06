use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;
use std::path::PathBuf;
use tracing::info;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::new();

const DATABASE_URL: &str = "./leaf_backend.db";

pub async fn init_db() -> Result<(), DbErr> {
    let url = PathBuf::from(DATABASE_URL);
    if !url.exists() {
        std::fs::File::create(DATABASE_URL).unwrap();
        info!("[DB] created, url: {:?}", DATABASE_URL);
    }

    let url = url.canonicalize().unwrap();

    info!("[DB] initialized, url: {:?}", DATABASE_URL);
    let db = Database::connect(format!("sqlite://{}", url.to_str().unwrap())).await?;

    let schema_manager = SchemaManager::new(&db);
    super::migrator::Migrator::up(&db, None).await?;
    assert!(schema_manager.has_table("leaf_user").await?);

    DB.set(db).unwrap();
    Ok(())
}
