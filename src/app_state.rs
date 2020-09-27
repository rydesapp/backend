use crate::graphql_schema::Schema;

#[derive(Clone)]
pub struct AppState {
    pub schema: Schema,
}
