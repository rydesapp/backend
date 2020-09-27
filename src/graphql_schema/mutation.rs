use async_graphql::{Context, FieldResult};

use super::ContextData;

pub struct MutationRoot;
#[async_graphql::Object]
impl MutationRoot {
    // #[field(guard(PermissionGuard(permission = "Permission::CreateWaves")))]
    pub async fn mut_fn(&self, ctx: &Context<'_>) -> FieldResult<i32> {
        let _data = ctx.data::<ContextData>()?;
        Ok(21)
    }
}
