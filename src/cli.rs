use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "zquery", about = "Browse Zotero library for coding agents")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Path to Zotero data directory (overrides auto-detection)
    #[arg(long, env = "ZOTERO_DATA_DIR", global = true)]
    pub data_dir: Option<String>,

    /// Output format
    #[arg(long, value_enum, default_value = "table", global = true)]
    pub format: OutputFormat,
}

#[derive(Subcommand)]
pub enum Command {
    /// List collections or items at a path
    Ls {
        /// Collection path (e.g., "bert class/voter"). Empty = top-level.
        path: Option<String>,
    },
    /// Show detailed metadata for items in a collection
    Show {
        /// Collection path
        path: String,
    },
    /// Get the PDF file path for a specific item
    Pdf {
        /// Collection path
        path: String,
        /// Item selector: title substring or 1-based index from `ls` output
        #[arg(long)]
        item: String,
    },
    /// Copy PDFs to a local folder for agent access
    Export {
        /// Collection path
        path: String,
        /// Destination directory (default: ./zotero_pdfs)
        #[arg(long, default_value = "zotero_pdfs")]
        dest: String,
        /// Item selector: title substring or 1-based index (omit to export all)
        #[arg(long)]
        item: Option<String>,
    },
}

#[derive(ValueEnum, Clone, Copy)]
pub enum OutputFormat {
    Table,
    Json,
    Plain,
}
