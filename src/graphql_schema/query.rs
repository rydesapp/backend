use async_graphql::{Context, FieldResult};

use super::ContextData;
use crate::models::*;
use sqlx::{prelude::*, query_as};
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<User>>> {
        let data = ctx.data::<ContextData>()?;
        let rows =
            query_as::<_, User>(r"SELECT uuid, first_name, last_name, email, phone FROM users;")
                .fetch_all(&data.db.pool)
                .await?;
        Ok(Some(rows))
    }

    pub async fn user(&self, ctx: &Context<'_>, user_id: uuid::Uuid) -> FieldResult<Option<User>> {
        let data = ctx.data::<ContextData>()?;
        let row = query_as::<_, User>(
            r"SELECT uuid, first_name, last_name, email, phone FROM users WHERE uuid = $1;",
        )
        .bind(user_id)
        .fetch_one(&data.db.pool)
        .await?;
        Ok(Some(row))
    }
}
