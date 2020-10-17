use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> anyhow::Result<Database> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        let db = Database { pool };
        Ok(db)
    }
}
