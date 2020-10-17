use crate::database::Database;
use crate::graphql_schema::ContextData;
use anyhow::Result;
use argon2::{self, Config};
use async_graphql::{Context, FieldResult, InputObject, Object, SimpleObject};
use sqlx::query_as;
use uuid::Uuid;

#[derive(InputObject)]
pub struct UserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, sqlx::FromRow, SimpleObject)]
pub struct User {
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

impl User {
    pub async fn get_by_id(id: Uuid) -> Result<Option<User>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let db = Database::new(&database_url).await?;
        let user = query_as!(
            User,
            r"SELECT uuid, email, first_name, last_name, phone FROM users WHERE uuid = $1;",
            id
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(Some(user))
    }
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<User>>> {
        let data = ctx.data::<ContextData>()?;
        let users = query_as!(
            User,
            r"SELECT uuid, first_name, last_name, email, phone FROM users;"
        )
        .fetch_all(&data.db.pool)
        .await?;
        Ok(Some(users))
    }

    pub async fn user(&self, id: uuid::Uuid) -> FieldResult<Option<User>> {
        let user = User::get_by_id(id).await?;
        Ok(user)
    }

    pub async fn me(&self, ctx: &Context<'_>) -> FieldResult<Option<User>> {
        let current_user = ctx.data::<Option<User>>()?;
        Ok(current_user.clone())
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    // #[field(guard(PermissionGuard(permission = "Permission::CreateWaves")))]
    //
    pub async fn sign_up(&self, ctx: &Context<'_>, user: UserInput) -> FieldResult<User> {
        let data = ctx.data::<ContextData>()?;
        let config = Config::default();
        let key = std::env::var("PASSWORD_SALT")?;
        let hashed_password =
            argon2::hash_encoded(&user.password.into_bytes(), &key.into_bytes(), &config).unwrap();
        let user_row = query_as!(
            User,
            r#"
INSERT INTO users ( first_name, last_name, email, password )
VALUES ( $1, $2, $3, $4 )
RETURNING uuid, first_name, last_name, email, phone
        "#,
            user.first_name,
            user.last_name,
            user.email,
            hashed_password,
        )
        .fetch_one(&data.db.pool)
        .await?;
        Ok(user_row)
    }
    pub async fn _sign_in(&self, ctx: &Context<'_>, user: UserInput) -> FieldResult<User> {
        let data = ctx.data::<ContextData>()?;
        let config = Config::default();
        let key = std::env::var("PASSWORD_SALT")?;
        let hashed_password =
            argon2::hash_encoded(&user.password.into_bytes(), &key.into_bytes(), &config).unwrap();
        let user_row = query_as!(
            User,
            r#"
INSERT INTO users ( first_name, last_name, email, password )
VALUES ( $1, $2, $3, $4 )
RETURNING uuid, first_name, last_name, email, phone
        "#,
            user.first_name,
            user.last_name,
            user.email,
            hashed_password,
        )
        .fetch_one(&data.db.pool)
        .await?;
        Ok(user_row)
    }
}
