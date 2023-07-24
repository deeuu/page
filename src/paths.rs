use anyhow::{anyhow, Result};
use directories_next::ProjectDirs;
use std::path::Path;

/// Returns the path to the storage folder containing the `entries_file`
pub fn storage_dir() -> Result<String> {
    match std::env::var("PAGE_STORAGE_FOLDER") {
        Ok(f) => Ok(f),
        Err(_) => match ProjectDirs::from("", "", "page") {
            Some(pd) => {
                let dir = pd.data_dir().display().to_string();
                Ok(dir)
            }
            None => Err(anyhow!("couldn't determine project storage folder")),
        },
    }
}

/// Returns the path to the entries.toml.age file
pub fn entries_file() -> Result<String> {
    Ok(Path::new(&storage_dir()?)
        .join("entries.toml.age")
        .display()
        .to_string())
}

/// Returns the path toth the hooks directory
pub fn hooks_dir() -> Result<String> {
    Ok(Path::new(&storage_dir()?)
        .join("hooks")
        .display()
        .to_string())
}
