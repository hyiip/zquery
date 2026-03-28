use crate::models::{BrowseResult, ExportResult, Item};

use super::Renderer;

pub struct PlainRenderer;

impl Renderer for PlainRenderer {
    fn render_browse(&self, result: &BrowseResult) {
        match result {
            BrowseResult::Collections { collections, .. } => {
                for c in collections {
                    println!("{}", c.name);
                }
            }
            BrowseResult::Items { items, .. } => {
                for (i, item) in items.iter().enumerate() {
                    println!("{}. {} - {}", i + 1, item.cite_key(), item.title);
                }
            }
            BrowseResult::Mixed {
                subcollections,
                items,
                ..
            } => {
                for c in subcollections {
                    println!("[collection] {}", c.name);
                }
                for (i, item) in items.iter().enumerate() {
                    println!("{}. {} - {}", i + 1, item.cite_key(), item.title);
                }
            }
        }
    }

    fn render_show(&self, items: &[Item], _collection_name: &str) {
        for (i, item) in items.iter().enumerate() {
            println!("{}. {} - {}", i + 1, item.cite_key(), item.title);
            if let Some(doi) = &item.doi {
                println!("   DOI: {}", doi);
            }
            if let Some(pdf) = &item.pdf_path {
                println!("   PDF: {}", pdf);
            }
        }
    }

    fn render_pdf_path(&self, item: &Item) {
        match &item.pdf_path {
            Some(path) => println!("{}", path),
            None => eprintln!("No PDF for: {}", item.title),
        }
    }

    fn render_export(&self, result: &ExportResult) {
        for f in &result.exported {
            println!("{}", f.path);
        }
    }
}
