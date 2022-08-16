use crate::session::TcpSession;
use crate::{ConfigEntry, EntryKind};

use async_trait::async_trait;
use cdrs_tokio::query_values;
use cdrs_tokio::types::prelude::{Map, Row};
use cdrs_tokio::types::{AsRustType, ByName};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;
use std::option::Option;
use std::sync::Arc;
use std::vec::Vec;
use std::{fmt, result};

#[derive(Debug)]
pub struct DatabaseError {
    pub inner: String,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurred during the database operation.")
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[async_trait]
pub trait ConfigEntryStore {
    async fn list(&mut self, kind: EntryKind) -> Result<Vec<ConfigEntry>, DatabaseError>;
    async fn get(
        &mut self,
        kind: EntryKind,
        name: &String,
    ) -> result::Result<Option<ConfigEntry>, DatabaseError>;
    async fn insert(&mut self, entry: ConfigEntry) -> result::Result<(), DatabaseError>;
    async fn update(&mut self, entry: ConfigEntry) -> result::Result<(), DatabaseError>;
    async fn remove(&mut self, kind: EntryKind, name: &String)
        -> result::Result<(), DatabaseError>;
}

pub struct CassandraStore {
    /// Cassandra session that holds a pool of connections to nodes and provides an interface for
    /// interacting with the cluster.
    session: Arc<TcpSession>,
}

impl CassandraStore {
    pub fn new(session: Arc<TcpSession>) -> Self {
        Self { session }
    }

    fn from_row(&self, row: &Row) -> ConfigEntry {
        let name = row.r_by_name::<String>(&"name").unwrap();

        let kind = match row.r_by_name::<i32>(&"kind").unwrap() {
            1 => EntryKind::Kafka,
            _ => EntryKind::Unknown,
        };

        let meta: HashMap<String, String> = match row.r_by_name::<Map>(&"meta") {
            Ok(m) => m.as_r_type().unwrap(),
            Err(_) => HashMap::new(),
        };

        let created_at = row.r_by_name::<DateTime<Utc>>(&"created_at").unwrap();
        let updated_at = row.r_by_name::<DateTime<Utc>>(&"updated_at").unwrap();

        ConfigEntry::init(kind, name, meta, created_at, updated_at).unwrap()
    }
}

#[async_trait]
impl ConfigEntryStore for CassandraStore {
    async fn list(&mut self, kind: EntryKind) -> Result<Vec<ConfigEntry>, DatabaseError> {
        let rows = self
            .session
            .query_with_values(
                "SELECT * FROM registry.config_entries WHERE kind = ? LIMIT 100;",
                query_values!(kind as i32),
            )
            .await
            .expect("query")
            .response_body()
            .expect("get body")
            .into_rows()
            .expect("into rows");

        let mut entries = Vec::<ConfigEntry>::new();

        for row in rows {
            let entry = self.from_row(&row);
            entries.push(entry);
        }

        Ok(entries)
    }

    async fn get(
        &mut self,
        kind: EntryKind,
        name: &String,
    ) -> result::Result<Option<ConfigEntry>, DatabaseError> {
        let rows = self
            .session
            .query_with_values(
                "SELECT * FROM registry.config_entries WHERE kind = ? AND name = ?;",
                query_values!(kind as i32, name.to_owned()),
            )
            .await
            .expect("query")
            .response_body()
            .expect("get body")
            .into_rows()
            .expect("into rows");

        if rows.len() == 0 {
            return Ok(None);
        }

        let row = rows.get(0).unwrap();
        Ok(Some(self.from_row(row)))
    }

    async fn insert(&mut self, entry: ConfigEntry) -> result::Result<(), DatabaseError> {
        let query = "
            INSERT INTO registry.config_entries (
                kind, 
                name, 
                meta, 
                created_at, 
                updated_at)
            VALUES (?, ?, ?, ?, ?);";

        let result = self
            .session
            .query_with_values(
                query,
                query_values!(
                    entry.kind as i32,
                    entry.name,
                    entry.meta,
                    entry.created_at,
                    entry.updated_at
                ),
            )
            .await;

        if result.is_ok() {
            return Ok(());
        }

        Err(DatabaseError {
            inner: result.unwrap_err().to_string(),
        })
    }

    async fn update(&mut self, entry: ConfigEntry) -> result::Result<(), DatabaseError> {
        let query = "
            UPDATE registry.config_entries 
            SET meta = ?, modify_timestamp = ?
            WHERE kind = ? AND name = ?;";

        let result = self
            .session
            .query_with_values(
                query,
                query_values!(entry.meta, entry.updated_at, entry.kind as i32, entry.name),
            )
            .await;

        if result.is_ok() {
            return Ok(());
        }

        Err(DatabaseError {
            inner: result.unwrap_err().to_string(),
        })
    }

    async fn remove(
        &mut self,
        kind: EntryKind,
        name: &String,
    ) -> result::Result<(), DatabaseError> {
        let result = self
            .session
            .query_with_values(
                "DELETE FROM registry.config_entries WHERE kind = ? AND name = ?;",
                query_values!(kind as i32, name.to_owned()),
            )
            .await;

        if result.is_ok() {
            return Ok(());
        }

        Err(DatabaseError {
            inner: result.unwrap_err().to_string(),
        })
    }
}
