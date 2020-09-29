use async_graphql::{Context, FieldResult};

use super::ContextData;
use crate::models::*;
use sqlx::query_as;
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<User>>> {
        let data = ctx.data::<ContextData>()?;
        let rows = query_as!(
            User,
            "SELECT uuid, first_name, last_name, email, phone FROM users;"
        )
        .fetch_all(&data.db.pool)
        .await?;
        Ok(Some(rows))
    }
}
