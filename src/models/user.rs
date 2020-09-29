use async_graphql::*;
use uuid::Uuid;

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
