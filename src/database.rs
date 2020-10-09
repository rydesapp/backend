use sqlx::postgres::PgPool;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> anyhow::Result<Database> {
        let pool = PgPool::builder().max_size(5).build(database_url).await?;
        let db = Database { pool };
        Ok(db)
    }
}
