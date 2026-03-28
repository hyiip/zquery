pub mod json;
pub mod plain;
pub mod table;

use crate::cli::OutputFormat;
use crate::models::{BrowseResult, ExportResult, Item};

pub trait Renderer {
    fn render_browse(&self, result: &BrowseResult);
    fn render_show(&self, items: &[Item], collection_name: &str);
    fn render_pdf_path(&self, item: &Item);
    fn render_export(&self, result: &ExportResult);
}

pub fn create_renderer(format: OutputFormat) -> Box<dyn Renderer> {
    match format {
        OutputFormat::Table => Box::new(table::TableRenderer),
        OutputFormat::Json => Box::new(json::JsonRenderer),
        OutputFormat::Plain => Box::new(plain::PlainRenderer),
    }
}
