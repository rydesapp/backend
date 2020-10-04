use anyhow::Result;
use async_graphql::*;
use sqlx::query_as;
use uuid::Uuid;

use crate::database::Database;
#[derive(GQLInputObject)]
pub struct UserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[Object]
impl User {
    pub async fn id(&self) -> Uuid {
        self.uuid
    }
}

impl User {
    pub async fn get_by_id(id: Uuid) -> Result<Option<User>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let dbpool = Database::new(&database_url).await?;

        let row = query_as!(
            User,
            r"SELECT uuid, email, first_name, last_name, phone FROM users WHERE uuid = $1;",
            id
        )
        .fetch_one(&dbpool.pool)
        .await?;

        Ok(Some(row))
    }
}
