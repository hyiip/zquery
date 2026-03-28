use crate::models::{BrowseResult, ExportResult, Item};

use super::Renderer;

pub struct JsonRenderer;

impl Renderer for JsonRenderer {
    fn render_browse(&self, result: &BrowseResult) {
        println!("{}", serde_json::to_string_pretty(result).unwrap());
    }

    fn render_show(&self, items: &[Item], _collection_name: &str) {
        println!("{}", serde_json::to_string_pretty(items).unwrap());
    }

    fn render_pdf_path(&self, item: &Item) {
        let output = serde_json::json!({
            "title": item.title,
            "cite_key": item.cite_key(),
            "pdf_path": item.pdf_path,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn render_export(&self, result: &ExportResult) {
        println!("{}", serde_json::to_string_pretty(result).unwrap());
    }
}
