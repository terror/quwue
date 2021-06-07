// stdlib
pub(crate) use std::convert::Infallible;

// dependencies
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use twilight_model::id::{MessageId, UserId};
pub(crate) use url::Url;

// local dependencies
pub(crate) use model::{Action, Emoji, Prompt, PromptMessage, Update, User};

// modules
pub(crate) use crate::error;

// traits
pub(crate) use crate::{unwrap_infallible::UnwrapInfallible, value::Value};

// structs and enums
pub(crate) use crate::{db::Db, error::Error, update_tx::UpdateTx};

// type aliases
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
pub(crate) type Transaction<'a> = sqlx::Transaction<'a, sqlx::Sqlite>;