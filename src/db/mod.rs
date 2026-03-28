pub mod attachments;
pub mod collections;
pub mod items;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::PathBuf;

use crate::config::Config;

pub struct ZoteroDB {
    pub conn: Connection,
    pub data_dir: PathBuf,
}

impl ZoteroDB {
    pub fn open(config: &Config) -> Result<Self> {
        let db_path = config.db_path.display().to_string().replace('\\', "/");
        let uri = format!("file:{}?mode=ro&immutable=1", db_path);
        let conn = Connection::open_with_flags(
            &uri,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_URI,
        )
        .with_context(|| {
            format!(
                "Failed to open Zotero database at {}. Is the path correct?",
                config.db_path.display()
            )
        })?;

        Ok(ZoteroDB {
            conn,
            data_dir: config.data_dir.clone(),
        })
    }

    pub fn storage_dir(&self) -> PathBuf {
        self.data_dir.join("storage")
    }
}
