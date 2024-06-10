use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn get_db_pool() -> sqlx::Result<sqlx::PgPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
