use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::data::DbId;
#[derive(Debug, Clone, Constructor, Serialize, Deserialize, PartialEq)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ClipId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_id_new() {
        let db_id: DbId = DbId::new();
        let clip_id = ClipId::new(db_id.clone());
        assert_eq!(clip_id.into_inner(), db_id);
    }

    #[test]
    fn test_clip_id_from() {
        let db_id = DbId::new();
        let clip_id = ClipId::from(db_id.clone());
        assert_eq!(clip_id.into_inner(), db_id);
    }

    #[test]
    fn test_clip_id_default() {
        let clip_id = ClipId::default();
        assert_eq!(clip_id.into_inner(), DbId::nil());
    }
}
