use super::ContextData;
use crate::models::*;
use argon2::{self, Config};
use async_graphql::{Context, FieldResult, Object};
use sqlx::{prelude::*, query_as};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // #[field(guard(PermissionGuard(permission = "Permission::CreateWaves")))]
    pub async fn mut_fn(&self, ctx: &Context<'_>) -> FieldResult<i32> {
        let _data = ctx.data::<ContextData>()?;
        Ok(21)
    }
    pub async fn sign_up(&self, ctx: &Context<'_>, user: UserInput) -> FieldResult<User> {
        let data = ctx.data::<ContextData>()?;
        let config = Config::default();
        let key = std::env::var("PASSWORD_SALT")?;
        let hashed_password =
            argon2::hash_encoded(&user.password.into_bytes(), &key.into_bytes(), &config).unwrap();
        let user_row = query_as::<_, User>(
            r#"
INSERT INTO users ( first_name, last_name, email, password )
VALUES ( $1, $2, $3, $4 )
RETURNING uuid, first_name, last_name, email, phone
        "#,
        )
        .bind(user.first_name)
        .bind(user.last_name)
        .bind(user.email)
        .bind(hashed_password)
        .fetch_one(&data.db.pool)
        .await?;
        Ok(user_row)
    }
}
