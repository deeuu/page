use crate::paths::{hooks_dir, storage_dir};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Represents callable scripts which can be triggered at certain times
pub enum Hook {
    PreLoad,
    PostSave,
}

impl Hook {
    fn name(&self) -> String {
        match *self {
            Self::PreLoad => "pre_load".to_string(),
            Self::PostSave => "post_save".to_string(),
        }
    }
}

/// Represents events which can trigger hooks
#[derive(Debug)]
pub enum HookEvent {
    NewEntry,
    ListEntries,
    ShowEntry,
    EditEntry,
    RemoveEntry,
}

impl HookEvent {
    fn name(&self) -> String {
        match *self {
            Self::NewEntry => "new_entry".to_string(),
            Self::ListEntries => "list_entries".to_string(),
            Self::ShowEntry => "show_entry".to_string(),
            Self::EditEntry => "edit_entry".to_string(),
            Self::RemoveEntry => "remove_entry".to_string(),
        }
    }
}

pub fn run_hook(hook: &Hook, event: &HookEvent) -> Result<()> {
    let path = Path::new(&hooks_dir()?)
        .join(hook.name())
        .display()
        .to_string();
    if fs::metadata(&path).is_ok() {
        println!("Running {} hook", hook.name());
        let storage_dir = storage_dir()?;
        let output = Command::new(path)
            .args(&[event.name()])
            .current_dir(storage_dir)
            .output()?;

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        for line in stdout.lines() {
            println!("{}: {}", hook.name(), line);
        }
        for line in stderr.lines() {
            println!("{}: {}", hook.name(), line);
        }

        if !output.status.success() {
            return Err(anyhow!("{} hook failed", hook.name()));
        }
    }

    Ok(())
}
