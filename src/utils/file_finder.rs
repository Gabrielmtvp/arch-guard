use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_php_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
    {
        let path = entry.path();

        // Only include .php files
        if path.extension().and_then(|s| s.to_str()) == Some("php") {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}