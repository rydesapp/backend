use crate::database::Database;
use anyhow::{anyhow, Result};
use async_session::{Session, SessionStore};
use async_sqlx_session::PostgresSessionStore;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, query_as};

#[derive(Deserialize, Serialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(
    email: String,
    password: String,
    session_store: &PostgresSessionStore,
) -> Result<String> {
    let database_url = std::env::var("DATABASE_URL")?;
    let dbpool = Database::new(&database_url).await?;

    #[derive(sqlx::FromRow)]
    struct UserInfo {
        uuid: uuid::Uuid,
        password: String,
    }

    let row = query_as::<_, UserInfo>(r"SELECT uuid, password FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(&dbpool.pool)
        .await?;
    let password_correct = argon2::verify_encoded(&row.password, &password.clone().into_bytes())?;
    if password_correct {
        let mut session = Session::new();
        session.insert("user_id", row.uuid)?;
        session.expire_in(chrono::Duration::days(1).to_std()?);
        let cookie_value = session_store.store_session(session).await?.unwrap();
        return Ok(cookie_value);
    }
    Err(anyhow!("Authentication failed!"))
}
