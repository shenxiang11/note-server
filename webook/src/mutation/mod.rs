pub(crate) mod note;
mod user;

use crate::mutation::note::NoteMutation;
use crate::mutation::user::UserMutation;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation, NoteMutation);
