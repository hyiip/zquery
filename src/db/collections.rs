use anyhow::{bail, Result};
use rusqlite::params;

use super::ZoteroDB;
use crate::models::Collection;

impl ZoteroDB {
    pub fn get_top_level_collections(&self) -> Result<Vec<Collection>> {
        let mut stmt = self.conn.prepare(
            "SELECT collectionID, collectionName, parentCollectionID
             FROM collections
             WHERE parentCollectionID IS NULL
             ORDER BY collectionName COLLATE NOCASE",
        )?;

        let collections: Vec<Collection> = stmt
            .query_map([], |row| {
                Ok(Collection {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parent_id: row.get(2)?,
                    subcollection_count: 0,
                    item_count: 0,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        self.enrich_collections(collections)
    }

    pub fn get_subcollections(&self, parent_id: i64) -> Result<Vec<Collection>> {
        let mut stmt = self.conn.prepare(
            "SELECT collectionID, collectionName, parentCollectionID
             FROM collections
             WHERE parentCollectionID = ?
             ORDER BY collectionName COLLATE NOCASE",
        )?;

        let collections: Vec<Collection> = stmt
            .query_map(params![parent_id], |row| {
                Ok(Collection {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parent_id: row.get(2)?,
                    subcollection_count: 0,
                    item_count: 0,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        self.enrich_collections(collections)
    }

    pub fn resolve_path(&self, path: &str) -> Result<i64> {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if segments.is_empty() {
            bail!("Empty collection path");
        }

        let mut current_id = self.find_collection_by_name_and_parent(segments[0], None)?;

        for segment in &segments[1..] {
            current_id = self.find_collection_by_name_and_parent(segment, Some(current_id))?;
        }

        Ok(current_id)
    }

    pub fn get_collection_name(&self, collection_id: i64) -> Result<String> {
        let name: String = self.conn.query_row(
            "SELECT collectionName FROM collections WHERE collectionID = ?",
            params![collection_id],
            |row| row.get(0),
        )?;
        Ok(name)
    }

    fn find_collection_by_name_and_parent(
        &self,
        name: &str,
        parent_id: Option<i64>,
    ) -> Result<i64> {
        let result = match parent_id {
            None => self.conn.query_row(
                "SELECT collectionID FROM collections
                 WHERE collectionName = ? COLLATE NOCASE
                 AND parentCollectionID IS NULL",
                params![name],
                |row| row.get::<_, i64>(0),
            ),
            Some(pid) => self.conn.query_row(
                "SELECT collectionID FROM collections
                 WHERE collectionName = ? COLLATE NOCASE
                 AND parentCollectionID = ?",
                params![name, pid],
                |row| row.get::<_, i64>(0),
            ),
        };

        match result {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let available = self.list_siblings(parent_id)?;
                let parent_desc = match parent_id {
                    None => "top level".to_string(),
                    Some(pid) => {
                        self.get_collection_name(pid)
                            .unwrap_or_else(|_| format!("ID {}", pid))
                    }
                };
                bail!(
                    "Collection '{}' not found under {}. Available: {}",
                    name,
                    parent_desc,
                    available.join(", ")
                );
            }
            Err(e) => Err(e.into()),
        }
    }

    fn list_siblings(&self, parent_id: Option<i64>) -> Result<Vec<String>> {
        let names: Vec<String> = match parent_id {
            None => {
                let mut stmt = self.conn.prepare(
                    "SELECT collectionName FROM collections
                     WHERE parentCollectionID IS NULL
                     ORDER BY collectionName COLLATE NOCASE",
                )?;
                stmt.query_map([], |row| row.get(0))?
                    .collect::<Result<Vec<_>, _>>()?
            }
            Some(pid) => {
                let mut stmt = self.conn.prepare(
                    "SELECT collectionName FROM collections
                     WHERE parentCollectionID = ?
                     ORDER BY collectionName COLLATE NOCASE",
                )?;
                stmt.query_map(params![pid], |row| row.get(0))?
                    .collect::<Result<Vec<_>, _>>()?
            }
        };
        Ok(names)
    }

    fn enrich_collections(&self, mut collections: Vec<Collection>) -> Result<Vec<Collection>> {
        if collections.is_empty() {
            return Ok(collections);
        }

        let ids: Vec<i64> = collections.iter().map(|c| c.id).collect();
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let params: Vec<&dyn rusqlite::types::ToSql> =
            ids.iter().map(|id| id as &dyn rusqlite::types::ToSql).collect();

        // Batch subcollection counts
        let sql = format!(
            "SELECT parentCollectionID, COUNT(*)
             FROM collections
             WHERE parentCollectionID IN ({})
             GROUP BY parentCollectionID",
            placeholders
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let sub_counts: std::collections::HashMap<i64, usize> = stmt
            .query_map(params.as_slice(), |row| Ok((row.get::<_, i64>(0)?, row.get::<_, usize>(1)?)))?
            .collect::<Result<_, _>>()?;

        // Batch item counts
        let sql = format!(
            "SELECT ci.collectionID, COUNT(*)
             FROM collectionItems ci
             JOIN items i ON i.itemID = ci.itemID
             JOIN itemTypes it ON it.itemTypeID = i.itemTypeID
             WHERE ci.collectionID IN ({})
             AND it.typeName NOT IN ('attachment', 'note')
             AND i.itemID NOT IN (SELECT itemID FROM deletedItems)
             GROUP BY ci.collectionID",
            placeholders
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let item_counts: std::collections::HashMap<i64, usize> = stmt
            .query_map(params.as_slice(), |row| Ok((row.get::<_, i64>(0)?, row.get::<_, usize>(1)?)))?
            .collect::<Result<_, _>>()?;

        for c in &mut collections {
            c.subcollection_count = sub_counts.get(&c.id).copied().unwrap_or(0);
            c.item_count = item_counts.get(&c.id).copied().unwrap_or(0);
        }

        Ok(collections)
    }
}
