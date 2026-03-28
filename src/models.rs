use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub subcollection_count: usize,
    pub item_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub id: i64,
    pub key: String,
    pub item_type: String,
    pub title: String,
    pub year: Option<String>,
    pub creators: Vec<Creator>,
    pub date: Option<String>,
    pub abstract_note: Option<String>,
    pub doi: Option<String>,
    pub url: Option<String>,
    pub publication_title: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub pdf_path: Option<String>,
}

impl Item {
    pub fn authors_short(&self) -> String {
        let authors: Vec<&Creator> = self
            .creators
            .iter()
            .filter(|c| c.creator_type == "author")
            .collect();
        match authors.len() {
            0 => "Unknown".to_string(),
            1 => authors[0].last_name.clone(),
            2 => format!("{}, {}", authors[0].last_name, authors[1].last_name),
            _ => format!("{} et al.", authors[0].last_name),
        }
    }

    pub fn cite_key(&self) -> String {
        let year = self.year.as_deref().unwrap_or("n.d.");
        format!("{} ({})", self.authors_short(), year)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Creator {
    pub first_name: String,
    pub last_name: String,
    pub creator_type: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum BrowseResult {
    Collections {
        path: String,
        collections: Vec<Collection>,
    },
    Items {
        path: String,
        collection: String,
        items: Vec<Item>,
    },
    Mixed {
        path: String,
        collection: String,
        subcollections: Vec<Collection>,
        items: Vec<Item>,
    },
}

#[derive(Debug, Serialize)]
pub struct ExportResult {
    pub exported: Vec<ExportedFile>,
}

#[derive(Debug, Serialize)]
pub struct ExportedFile {
    pub title: String,
    pub cite_key: String,
    pub path: String,
}
