#![cfg(feature = "ssr")]

use sqlx::*;
use tracing::info;

static DB: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();

async fn create_pool() -> sqlx::PgPool {
    info!("Creating database pool");
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    info!("Migrating DB");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");

    pool
}
pub async fn init_db() -> Result<(), sqlx::Pool<sqlx::Postgres>> {
    DB.set(create_pool().await)
}

pub fn get_db<'a>() -> &'a sqlx::PgPool {
    info!("Getting database pool");
    DB.get().expect("database unitialized")
}
