use crate::cli::EntryAttribute;
use crate::entries::{load_entries, save_entries, Entry, Storage};
use crate::hooks::{run_hook, Hook, HookEvent};
use crate::paths::{entries_file, hooks_dir, storage_dir};
use crate::utilities;
use age::secrecy::{ExposeSecret, SecretString};
use anyhow::{anyhow, Error, Result};
use std::fs;

pub fn init(no_keyring: bool) -> Result<(), Error> {
    fs::create_dir_all(storage_dir()?)?;
    let path = entries_file()?;
    if fs::metadata(&path).is_err() {
        fs::File::create(entries_file()?)?;
        println!("Created entries file {}", path);
        let passphrase = utilities::get_passphrase(no_keyring)?;
        let entries: Storage = toml::from_str("")?;
        save_entries(passphrase, &entries)?
    } else {
        println!("Entries file {} already exists", path);
    }
    Ok(())
}

pub fn new_entry(
    entry: String,
    username: Option<String>,
    url: Option<String>,
    no_keyring: bool,
) -> Result<(), Error> {
    run_hook(&Hook::PreLoad, &HookEvent::NewEntry)?;
    let passphrase = utilities::get_passphrase(no_keyring)?;
    let mut storage = load_entries(passphrase.clone())?;

    if storage.entries.contains_key(&entry) {
        let overwrite = utilities::read_stdin(&format!(
            "Entry '{}' already exists. Overwrite (y/N)?",
            entry
        ))?;
        if overwrite.to_uppercase() != "Y" {
            return Ok(());
        }
    }

    let password = SecretString::from(rpassword::prompt_password_stdout(&format!(
        "Password for '{}': ",
        entry
    ))?);

    storage.entries.insert(
        entry,
        Entry {
            password: password.expose_secret().to_string(),
            username,
            url,
        },
    );

    save_entries(passphrase, &storage)?;
    run_hook(&Hook::PostSave, &HookEvent::NewEntry)?;

    Ok(())
}

pub fn list(no_keyring: bool) -> Result<(), Error> {
    run_hook(&Hook::PreLoad, &HookEvent::ListEntries)?;

    let passphrase = utilities::get_passphrase(no_keyring)?;
    let storage = load_entries(passphrase)?;
    for name in storage.entries.keys() {
        println!("{}", name);
    }
    Ok(())
}

pub fn show(
    entry: &str,
    attribute: EntryAttribute,
    on_screen: bool,
    no_keyring: bool,
) -> Result<()> {
    run_hook(&Hook::PreLoad, &HookEvent::ShowEntry)?;
    let passphrase = utilities::get_passphrase(no_keyring)?;
    let storage = load_entries(passphrase)?;

    if storage.entries.contains_key(entry) {
        let entry = storage
            .entries
            .get(entry)
            .ok_or_else(|| anyhow!("entry '{}' not found", entry))?;

        match attribute {
            EntryAttribute::Password => {
                utilities::reveal(&entry.password, on_screen)?;
            }
            EntryAttribute::Username => {
                if let Some(u) = &entry.username {
                    utilities::reveal(u, on_screen)?;
                }
            }
            EntryAttribute::Url => {
                if let Some(u) = &entry.url {
                    utilities::reveal(u, on_screen)?;
                }
            }
        };
    } else {
        return Err(anyhow!("entry '{}' not found", entry));
    }

    Ok(())
}

pub fn edit(
    entry_name: String,
    new_name: Option<String>,
    username: Option<String>,
    url: Option<String>,
    no_prompt: bool,
    no_keyring: bool,
) -> Result<()> {
    run_hook(&Hook::PreLoad, &HookEvent::EditEntry)?;
    let passphrase = utilities::get_passphrase(no_keyring)?;
    let mut storage = load_entries(passphrase.clone())?;

    let entry = storage
        .entries
        .remove(&entry_name)
        .ok_or_else(|| anyhow!("entry '{}' not found", entry_name))?;

    let name = match new_name {
        Some(nm) => {
            // check if we are replacing an existing entry
            if (nm != entry_name) && (storage.entries.contains_key(&nm)) {
                let overwrite = utilities::read_stdin(&format!(
                    "Entry '{}' already exists. Overwrite (y/N)?",
                    nm
                ))?;

                if overwrite.to_uppercase() == "Y" {
                    let _ = storage.entries.remove(&nm);
                } else {
                    return Ok(());
                }
            }
            nm
        }
        None => entry_name,
    };

    let username = match username {
        Some(u) => Some(u),
        None => entry.username,
    };

    let url = match url {
        Some(u) => Some(u),
        None => entry.url,
    };

    let password = match no_prompt {
        true => entry.password,
        false => rpassword::prompt_password_stdout(&format!("Password for '{}': ", name))?,
    };

    storage.entries.insert(
        name,
        Entry {
            password,
            username,
            url,
        },
    );

    save_entries(passphrase, &storage)?;
    run_hook(&Hook::PostSave, &HookEvent::EditEntry)?;

    Ok(())
}

pub fn remove(entry: &str, no_keyring: bool) -> Result<()> {
    run_hook(&Hook::PreLoad, &HookEvent::RemoveEntry)?;
    let passphrase = utilities::get_passphrase(no_keyring)?;
    let mut storage = load_entries(passphrase.clone())?;
    if storage.entries.remove(entry).is_some() {
        save_entries(passphrase, &storage)?;
        run_hook(&Hook::PostSave, &HookEvent::RemoveEntry)?;
    } else {
        return Err(anyhow!("entry '{}' not found", entry));
    };

    Ok(())
}

pub fn info() -> Result<()> {
    let storage_path = entries_file()?;
    if fs::metadata(storage_path.clone()).is_ok() {
        println!("Storage file: {}", storage_path);
    } else {
        println!("Storage file doesn't exist yet, run `passge init` to create it");
    }

    let hooks_dir = hooks_dir()?;
    if fs::metadata(&hooks_dir).is_ok() {
        println!("Hooks directory: {}", hooks_dir);
    } else {
        println!("Hooks directory does not exist yet: {}", hooks_dir);
    }
    Ok(())
}

pub fn keyring_check() -> Result<()> {
    let username = &whoami::username();
    let keyring = utilities::new_keyring(username);
    if keyring.get_password().is_err() {
        return Err(anyhow!("Failed to access password in keyring"));
    }
    println!("Keyring integration seems fine");
    Ok(())
}

pub fn keyring_forget() -> Result<()> {
    let username = &whoami::username();
    let keyring = utilities::new_keyring(username);
    if keyring.delete_password().is_err() {
        return Err(anyhow!("Failed to delete password from keyring"));
    }
    Ok(())
}
