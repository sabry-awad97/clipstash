use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!(
                    "If the database has not yet been created, run: \n   $ sqlx database setup\n"
                );
                panic!("database connection error");
            }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Clone, Debug, From, Display, Deserialize, Serialize, PartialEq)]
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

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_db_id() {
        let id1 = DbId::new();
        let id2 = DbId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_nil_db_id() {
        let id = DbId::nil();
        assert_eq!(id, DbId(Uuid::nil()));
    }

    #[test]
    fn test_db_id_from_string() {
        let id_str = "01234567-89ab-cdef-0123-456789abcdef";
        let id = DbId::from_str(id_str).unwrap();
        assert_eq!(id, DbId(Uuid::parse_str(id_str).unwrap()));
    }

    #[test]
    fn test_db_id_to_string() {
        let id_str = "01234567-89ab-cdef-0123-456789abcdef";
        let id = DbId::from_str(id_str).unwrap();
        assert_eq!(String::from(id), id_str);
    }
}
