mod cli;
mod config;
mod db;
mod models;
mod output;

use anyhow::{bail, Result};
use clap::Parser;
use std::fs;
use std::path::Path;

use cli::{Cli, Command};
use config::Config;
use db::ZoteroDB;
use models::{BrowseResult, ExportResult, ExportedFile};
use output::create_renderer;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::detect(cli.data_dir.as_deref())?;
    let db = ZoteroDB::open(&config)?;
    let renderer = create_renderer(cli.format);

    match cli.command {
        Command::Ls { path } => cmd_ls(&db, path.as_deref(), &*renderer),
        Command::Show { path } => cmd_show(&db, &path, &*renderer),
        Command::Pdf { path, item } => cmd_pdf(&db, &path, &item, &*renderer),
        Command::Export { path, dest, item } => {
            cmd_export(&db, &path, &dest, item.as_deref(), &*renderer)
        }
    }
}

fn cmd_ls(db: &ZoteroDB, path: Option<&str>, renderer: &dyn output::Renderer) -> Result<()> {
    match path {
        None => {
            let collections = db.get_top_level_collections()?;
            renderer.render_browse(&BrowseResult::Collections {
                path: String::new(),
                collections,
            });
        }
        Some(path) => {
            let collection_id = db.resolve_path(path)?;
            let collection_name = db.get_collection_name(collection_id)?;
            let subcollections = db.get_subcollections(collection_id)?;
            let items = db.get_items_in_collection(collection_id)?;

            if subcollections.is_empty() {
                renderer.render_browse(&BrowseResult::Items {
                    path: path.to_string(),
                    collection: collection_name,
                    items,
                });
            } else if items.is_empty() {
                renderer.render_browse(&BrowseResult::Collections {
                    path: path.to_string(),
                    collections: subcollections,
                });
            } else {
                renderer.render_browse(&BrowseResult::Mixed {
                    path: path.to_string(),
                    collection: collection_name,
                    subcollections,
                    items,
                });
            }
        }
    }
    Ok(())
}

fn cmd_show(db: &ZoteroDB, path: &str, renderer: &dyn output::Renderer) -> Result<()> {
    let collection_id = db.resolve_path(path)?;
    let collection_name = db.get_collection_name(collection_id)?;
    let items = db.get_items_in_collection(collection_id)?;
    renderer.render_show(&items, &collection_name);
    Ok(())
}

fn cmd_pdf(
    db: &ZoteroDB,
    path: &str,
    selector: &str,
    renderer: &dyn output::Renderer,
) -> Result<()> {
    let collection_id = db.resolve_path(path)?;
    let items = db.get_items_in_collection(collection_id)?;
    let idx = db.find_item_by_selector(&items, selector)?;
    let item = &items[idx];

    if item.pdf_path.is_none() {
        bail!("No PDF attachment found for: {}", item.title);
    }

    renderer.render_pdf_path(item);
    Ok(())
}

fn cmd_export(
    db: &ZoteroDB,
    path: &str,
    dest: &str,
    selector: Option<&str>,
    renderer: &dyn output::Renderer,
) -> Result<()> {
    let collection_id = db.resolve_path(path)?;
    let items = db.get_items_in_collection(collection_id)?;

    let to_export: Vec<&models::Item> = if let Some(sel) = selector {
        let idx = db.find_item_by_selector(&items, sel)?;
        vec![&items[idx]]
    } else {
        items.iter().collect()
    };

    let dest_path = Path::new(dest);
    fs::create_dir_all(dest_path)?;

    let mut exported = Vec::new();
    for item in to_export {
        if let Some(src_path) = &item.pdf_path {
            let src = Path::new(src_path);
            if !src.exists() {
                eprintln!("Warning: PDF not found on disk for '{}': {}", item.title, src_path);
                continue;
            }

            let filename = sanitize_filename(&format!("{} - {}.pdf", item.cite_key(), item.title));
            let dest_file = dest_path.join(&filename);
            fs::copy(src, &dest_file)?;

            exported.push(ExportedFile {
                title: item.title.clone(),
                cite_key: item.cite_key(),
                path: dest_file.display().to_string(),
            });
        } else {
            eprintln!("Warning: No PDF attachment for '{}'", item.title);
        }
    }

    renderer.render_export(&ExportResult { exported });
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}
