use crate::database::Database;
use async_graphql::EmptySubscription;
use async_sqlx_session::PostgresSessionStore;
pub use mutation::MutationRoot;
pub use query::QueryRoot;

pub mod mutation;
pub mod query;

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct ContextData {
    pub db: Database,
    pub session_store: PostgresSessionStore,
}
