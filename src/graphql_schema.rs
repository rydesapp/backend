use crate::database::Database;
use crate::models::User;
use async_graphql::EmptySubscription;
pub use mutation::MutationRoot;
pub use query::QueryRoot;

pub mod mutation;
pub mod query;

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug)]
pub struct ContextData {
    pub db: Database,
    pub current_user: Option<User>,
}
