use std::collections::HashMap;

use anyhow::Result;
use rusqlite::params;

use super::ZoteroDB;
use crate::models::{Creator, Item};

impl ZoteroDB {
    pub fn get_items_in_collection(&self, collection_id: i64) -> Result<Vec<Item>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT i.itemID, i.key, it.typeName
             FROM items i
             JOIN collectionItems ci ON ci.itemID = i.itemID
             JOIN itemTypes it ON it.itemTypeID = i.itemTypeID
             WHERE ci.collectionID = ?
             AND it.typeName NOT IN ('attachment', 'note')
             AND i.itemID NOT IN (SELECT itemID FROM deletedItems)
             ORDER BY i.itemID",
        )?;

        let rows: Vec<(i64, String, String)> = stmt
            .query_map(params![collection_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        if rows.is_empty() {
            return Ok(Vec::new());
        }

        let item_ids: Vec<i64> = rows.iter().map(|(id, _, _)| *id).collect();
        let all_fields = self.batch_get_fields(&item_ids)?;
        let all_creators = self.batch_get_creators(&item_ids)?;
        let all_pdfs = self.batch_get_pdfs(&item_ids)?;

        let mut items = Vec::new();
        for (item_id, key, item_type) in rows {
            let fields = all_fields.get(&item_id).cloned().unwrap_or_default();

            let title = fields
                .iter()
                .find(|(k, _)| k == "title")
                .map(|(_, v)| v.clone())
                .unwrap_or_else(|| "[Untitled]".to_string());

            let date = fields
                .iter()
                .find(|(k, _)| k == "date")
                .map(|(_, v)| v.clone());

            let year = date.as_ref().and_then(|d| {
                let s = d.chars().take(4).collect::<String>();
                if s.chars().all(|c| c.is_ascii_digit()) && s.len() == 4 {
                    Some(s)
                } else {
                    None
                }
            });

            let find = |name: &str| -> Option<String> {
                fields
                    .iter()
                    .find(|(k, _)| k == name)
                    .map(|(_, v)| v.clone())
            };

            items.push(Item {
                id: item_id,
                key,
                item_type,
                title,
                year,
                creators: all_creators.get(&item_id).cloned().unwrap_or_default(),
                date,
                abstract_note: find("abstractNote"),
                doi: find("DOI"),
                url: find("url"),
                publication_title: find("publicationTitle"),
                volume: find("volume"),
                issue: find("issue"),
                pages: find("pages"),
                pdf_path: all_pdfs.get(&item_id).cloned(),
            });
        }

        Ok(items)
    }

    pub fn find_item_by_selector(&self, items: &[Item], selector: &str) -> Result<usize> {
        if let Ok(index) = selector.parse::<usize>() {
            if index >= 1 && index <= items.len() {
                return Ok(index - 1);
            }
            anyhow::bail!(
                "Index {} out of range. Collection has {} items.",
                index,
                items.len()
            );
        }

        let lower = selector.to_lowercase();
        let matches: Vec<usize> = items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.title.to_lowercase().contains(&lower))
            .map(|(i, _)| i)
            .collect();

        match matches.len() {
            0 => anyhow::bail!(
                "No item matching '{}'. Use 'zquery ls' to see available items.",
                selector
            ),
            1 => Ok(matches[0]),
            _ => {
                let titles: Vec<String> = matches
                    .iter()
                    .map(|&i| format!("  {}. {}", i + 1, items[i].title))
                    .collect();
                anyhow::bail!(
                    "Multiple items match '{}'. Be more specific:\n{}",
                    selector,
                    titles.join("\n")
                );
            }
        }
    }

    fn batch_get_fields(&self, item_ids: &[i64]) -> Result<HashMap<i64, Vec<(String, String)>>> {
        let placeholders = item_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT id.itemID, f.fieldName, idv.value
             FROM itemData id
             JOIN fields f ON f.fieldID = id.fieldID
             JOIN itemDataValues idv ON idv.valueID = id.valueID
             WHERE id.itemID IN ({})",
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
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut map: HashMap<i64, Vec<(String, String)>> = HashMap::new();
        for row in rows {
            let (item_id, field, value) = row?;
            map.entry(item_id).or_default().push((field, value));
        }
        Ok(map)
    }

    fn batch_get_creators(&self, item_ids: &[i64]) -> Result<HashMap<i64, Vec<Creator>>> {
        let placeholders = item_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT ic.itemID, c.firstName, c.lastName, ct.creatorType
             FROM itemCreators ic
             JOIN creators c ON c.creatorID = ic.creatorID
             JOIN creatorTypes ct ON ct.creatorTypeID = ic.creatorTypeID
             WHERE ic.itemID IN ({})
             ORDER BY ic.itemID, ic.orderIndex",
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
                Creator {
                    first_name: row.get(1)?,
                    last_name: row.get(2)?,
                    creator_type: row.get(3)?,
                },
            ))
        })?;

        let mut map: HashMap<i64, Vec<Creator>> = HashMap::new();
        for row in rows {
            let (item_id, creator) = row?;
            map.entry(item_id).or_default().push(creator);
        }
        Ok(map)
    }
}
