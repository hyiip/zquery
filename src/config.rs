use anyhow::{Result, bail};
use std::path::PathBuf;

pub struct Config {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
}

impl Config {
    pub fn detect(override_dir: Option<&str>) -> Result<Self> {
        let data_dir = if let Some(dir) = override_dir {
            PathBuf::from(dir)
        } else {
            Self::auto_detect()?
        };

        let db_path = data_dir.join("zotero.sqlite");
        if !db_path.exists() {
            bail!(
                "zotero.sqlite not found at {}. Use --data-dir or ZOTERO_DATA_DIR to specify your Zotero data directory.",
                db_path.display()
            );
        }

        Ok(Config { data_dir, db_path })
    }

    fn auto_detect() -> Result<PathBuf> {
        let candidates = Self::candidate_dirs();
        for candidate in &candidates {
            if candidate.join("zotero.sqlite").exists() {
                return Ok(candidate.clone());
            }
        }
        bail!(
            "Could not find Zotero data directory. Checked:\n{}\nUse --data-dir or ZOTERO_DATA_DIR to specify it.",
            candidates
                .iter()
                .map(|p| format!("  - {}", p.display()))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    fn candidate_dirs() -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if let Some(home) = dirs::home_dir() {
            dirs.push(home.join("Zotero"));
        }
        if let Ok(profile) = std::env::var("USERPROFILE") {
            let p = PathBuf::from(profile).join("Zotero");
            if !dirs.contains(&p) {
                dirs.push(p);
            }
        }
        dirs
    }
}
