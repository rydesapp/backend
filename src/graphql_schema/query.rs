use async_graphql::{Context, FieldResult};

use super::ContextData;
use crate::models::User;
use sqlx::query;
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<User<'_>>>> {
        let data = ctx.data::<ContextData>()?;
        let rows = query!("SELECT * FROM users;")
            .fetch_all(&data.db.pool)
            .await?;
        dbg!(rows);
        Ok(None)
    }
}
