use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}
