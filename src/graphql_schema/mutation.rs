use crate::models::*;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation);
