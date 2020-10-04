use crate::graphql_schema::Schema;
use async_sqlx_session::PostgresSessionStore;
#[derive(Clone)]
pub struct AppState {
    pub schema: Schema,
    pub session_store: PostgresSessionStore,
}
