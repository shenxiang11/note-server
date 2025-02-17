mod comment;
pub mod note;
mod user;

use crate::query::comment::CommentQuery;
use crate::query::note::NoteQuery;
use crate::query::user::UserQuery;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub(crate) struct QueryRoot(UserQuery, NoteQuery, CommentQuery);
