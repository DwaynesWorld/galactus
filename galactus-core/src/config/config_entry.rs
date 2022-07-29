use chrono::prelude::*;
use std::collections::HashMap;

#[repr(i32)]
#[derive(Clone, Debug, PartialEq)]
pub enum EntryKind {
    Unknown,
    Kafka,
}

impl EntryKind {
    pub fn to_str(&self) -> &str {
        match self {
            EntryKind::Kafka => "KAFKA",
            _ => "UNKNOWN",
        }
    }
}

impl TryFrom<i32> for EntryKind {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == EntryKind::Unknown as i32 => Ok(EntryKind::Unknown),
            x if x == EntryKind::Kafka as i32 => Ok(EntryKind::Kafka),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigEntry {
    /// The kind of configuration entry. Will always be Kafka.
    pub kind: EntryKind,

    /// Configuration name.
    pub name: String,

    /// A key/value pair collection of Kafka config entry options.
    pub meta: HashMap<String, String>,

    /// Represents the point in time in UTC Epoch time, when the entry was created.
    pub created_at: DateTime<Utc>,

    /// Represents the point in time in UTC Epoch time, when the entry was modified.
    pub updated_at: DateTime<Utc>,
}

impl ConfigEntry {
    pub fn new(
        kind: EntryKind,
        name: String,
        meta: HashMap<String, String>,
    ) -> Result<Self, &'static str> {
        Ok(ConfigEntry {
            kind,
            name,
            meta,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn init(
        kind: EntryKind,
        name: String,
        meta: HashMap<String, String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        Ok(ConfigEntry {
            kind,
            name,
            meta,
            created_at,
            updated_at,
        })
    }
}
