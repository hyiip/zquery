use comfy_table::{ContentArrangement, Table};

use crate::models::{BrowseResult, ExportResult, Item};

use super::Renderer;

pub struct TableRenderer;

impl Renderer for TableRenderer {
    fn render_browse(&self, result: &BrowseResult) {
        match result {
            BrowseResult::Collections { path, collections } => {
                println!(
                    "Collections at: {}",
                    if path.is_empty() { "/" } else { path }
                );
                let mut table = Table::new();
                table.set_content_arrangement(ContentArrangement::Dynamic);
                table.set_header(vec!["Name", "Subcollections", "Items"]);
                for c in collections {
                    table.add_row(vec![
                        c.name.clone(),
                        c.subcollection_count.to_string(),
                        c.item_count.to_string(),
                    ]);
                }
                println!("{table}");
            }
            BrowseResult::Items {
                path,
                collection: _,
                items,
            } => {
                println!("Items in: {} ({})", path, items.len());
                print_items_compact(items);
            }
            BrowseResult::Mixed {
                path,
                collection: _,
                subcollections,
                items,
            } => {
                if !subcollections.is_empty() {
                    println!("Subcollections in: {}", path);
                    let mut table = Table::new();
                    table.set_content_arrangement(ContentArrangement::Dynamic);
                    table.set_header(vec!["Name", "Subcollections", "Items"]);
                    for c in subcollections {
                        table.add_row(vec![
                            c.name.clone(),
                            c.subcollection_count.to_string(),
                            c.item_count.to_string(),
                        ]);
                    }
                    println!("{table}");
                }
                if !items.is_empty() {
                    println!("\nItems in: {} ({})", path, items.len());
                    print_items_compact(items);
                }
            }
        }
    }

    fn render_show(&self, items: &[Item], collection_name: &str) {
        println!(
            "Detailed metadata for: {} ({} items)\n",
            collection_name,
            items.len()
        );
        for (i, item) in items.iter().enumerate() {
            println!("{}. {}", i + 1, item.title);
            println!("   Authors: {}", format_creators_full(&item.creators));
            println!("   Year: {}", item.year.as_deref().unwrap_or("n.d."));
            println!("   Type: {}", item.item_type);
            if let Some(pub_title) = &item.publication_title {
                println!("   Journal: {}", pub_title);
            }
            if let Some(vol) = &item.volume {
                let issue = item.issue.as_deref().unwrap_or("");
                let pages = item.pages.as_deref().unwrap_or("");
                println!(
                    "   Volume: {}{}{}",
                    vol,
                    if !issue.is_empty() {
                        format!("({})", issue)
                    } else {
                        String::new()
                    },
                    if !pages.is_empty() {
                        format!(", pp. {}", pages)
                    } else {
                        String::new()
                    },
                );
            }
            if let Some(doi) = &item.doi {
                println!("   DOI: {}", doi);
            }
            if let Some(url) = &item.url {
                println!("   URL: {}", url);
            }
            if let Some(abs) = &item.abstract_note {
                let truncated = if abs.len() > 300 {
                    format!("{}...", &abs[..300])
                } else {
                    abs.clone()
                };
                println!("   Abstract: {}", truncated);
            }
            if let Some(pdf) = &item.pdf_path {
                println!("   PDF: {}", pdf);
            }
            println!();
        }
    }

    fn render_pdf_path(&self, item: &Item) {
        match &item.pdf_path {
            Some(path) => println!("{}", path),
            None => eprintln!("No PDF attachment found for: {}", item.title),
        }
    }

    fn render_export(&self, result: &ExportResult) {
        for f in &result.exported {
            println!("Exported: {} -> {}", f.cite_key, f.path);
        }
        println!("\n{} file(s) exported.", result.exported.len());
    }
}

fn print_items_compact(items: &[Item]) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["#", "Authors", "Year", "Title", "PDF"]);
    for (i, item) in items.iter().enumerate() {
        table.add_row(vec![
            (i + 1).to_string(),
            item.authors_short(),
            item.year.clone().unwrap_or_else(|| "n.d.".to_string()),
            item.title.clone(),
            if item.pdf_path.is_some() {
                "yes".to_string()
            } else {
                "no".to_string()
            },
        ]);
    }
    println!("{table}");
}

fn format_creators_full(creators: &[crate::models::Creator]) -> String {
    if creators.is_empty() {
        return "Unknown".to_string();
    }
    creators
        .iter()
        .map(|c| {
            if c.first_name.is_empty() {
                c.last_name.clone()
            } else {
                format!("{}, {}", c.last_name, c.first_name)
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}
