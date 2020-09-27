use uuid::Uuid;
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User<'a> {
    pub uuid: Uuid,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
}

#[async_graphql::Object]
impl User<'_> {
    pub async fn id(&self) -> Uuid {
        self.uuid
    }
}
