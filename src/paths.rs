use anyhow::{anyhow, Result};
use directories_next::ProjectDirs;
use std::path::{PathBuf};

/// Returns the path to the storage folder containing the `entries_file`
pub fn storage_dir() -> Result<String> {
    let folder = match std::env::var("PAGE_STORAGE_FOLDER") {
        Ok(f) => Some(PathBuf::from(f)),
        Err(_) => ProjectDirs::from("", "", "page")
            .map(|pd| pd.data_dir().to_path_buf()),
    };

    match folder {
        Some(p) => {
            let abs = p.canonicalize().unwrap_or(p);
            Ok(abs.display().to_string())
        },
        None => Err(anyhow!("Failed to find storage directory")),
    }
}

/// Returns the path to the entries.toml.age file
pub fn entries_file() -> Result<String> {
    Ok(PathBuf::from(storage_dir()?)
        .join("entries.toml.age")
        .display()
        .to_string())
}

/// Returns the path to the hooks directory
pub fn hooks_dir() -> Result<String> {
    Ok(PathBuf::from(storage_dir()?)
        .join("hooks")
        .display()
        .to_string())
}
