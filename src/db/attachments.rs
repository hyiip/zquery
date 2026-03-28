use std::collections::HashMap;

use anyhow::Result;

use super::ZoteroDB;

impl ZoteroDB {
    pub fn batch_get_pdfs(&self, item_ids: &[i64]) -> Result<HashMap<i64, String>> {
        let placeholders = item_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT ia.parentItemID, ia.path, ia.linkMode, i.key
             FROM itemAttachments ia
             JOIN items i ON i.itemID = ia.itemID
             WHERE ia.parentItemID IN ({})
             AND ia.contentType = 'application/pdf'",
            placeholders
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let params: Vec<&dyn rusqlite::types::ToSql> = item_ids
            .iter()
            .map(|id| id as &dyn rusqlite::types::ToSql)
            .collect();
        let rows = stmt.query_map(params.as_slice(), |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;

        let mut map: HashMap<i64, String> = HashMap::new();
        for row in rows {
            let (parent_id, raw_path, link_mode, key) = row?;
            if let Some(path) = raw_path {
                // Only keep the first PDF per item
                map.entry(parent_id)
                    .or_insert_with(|| self.resolve_attachment_path(&path, link_mode, &key));
            }
        }
        Ok(map)
    }

    fn resolve_attachment_path(&self, raw_path: &str, link_mode: i32, key: &str) -> String {
        if let Some(filename) = raw_path.strip_prefix("storage:") {
            let mut path = self.storage_dir();
            path.push(key);
            path.push(filename);
            path.display().to_string()
        } else if link_mode == 2 {
            raw_path.to_string()
        } else {
            let mut path = self.storage_dir();
            path.push(key);
            path.push(raw_path);
            path.display().to_string()
        }
    }
}
