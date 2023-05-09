use crate::entries::{load_entries, save_entries, Entry, Storage};
use crate::hooks::{run_hook, Hook, HookEvent};
use crate::paths::{entries_file, hooks_dir, storage_dir};
use crate::utilities;
use anyhow::{anyhow, Error, Result};
use secrecy::{ExposeSecret, Secret};
use std::fs;
use std::io;
use std::io::Write;

pub fn init(no_keyring: bool) -> Result<(), Error> {
    fs::create_dir_all(storage_dir()?)?;
    let path = entries_file()?;
    if fs::metadata(path).is_err() {
        fs::File::create(entries_file()?)?;
        let passphrase = utilities::get_passphrase("Passphrase: ", no_keyring)?;
        let entries: Storage = toml::from_str("")?;
        save_entries(passphrase, &entries)?
    }
    Ok(())
}

pub fn new_entry(no_keyring: bool) -> Result<(), Error> {
    run_hook(&Hook::PreLoad, &HookEvent::NewEntry)?;
    let passphrase = utilities::get_passphrase("Passphrase: ", no_keyring)?;
    let mut storage = load_entries(&passphrase)?;

    print!("New entry: ");
    io::stdout().flush()?;
    let mut entry = String::new();
    io::stdin().read_line(&mut entry)?;
    let entry = entry.trim();

    if storage.entries.contains_key(entry) {
        print!("'{}' already exists. Overwrite (y/N)? ", entry);
        io::stdout().flush()?;
        let mut overwrite = String::new();
        io::stdin().read_line(&mut overwrite)?;
        let overwrite = overwrite.trim();
        if overwrite.to_uppercase() != "Y" {
            return Ok(());
        }
    }

    let password = Secret::new(rpassword::prompt_password_stdout(&format!(
        "Password for {}: ",
        entry
    ))?);

    storage.entries.insert(
        entry.to_owned(),
        Entry {
            password: password.expose_secret().to_string(),
        },
    );

    save_entries(passphrase, &storage)?;
    run_hook(&Hook::PostSave, &HookEvent::NewEntry)?;

    Ok(())
}

pub fn list(no_keyring: bool) -> Result<(), Error> {
    run_hook(&Hook::PreLoad, &HookEvent::ListEntries)?;

    let passphrase = utilities::get_passphrase("Enter passphrase: ", no_keyring)?;
    let storage = load_entries(&passphrase)?;
    for name in storage.entries.keys() {
        println!("{}", name);
    }
    Ok(())
}

pub fn show(entry: &str, on_screen: bool, no_keyring: bool) -> Result<()> {
    run_hook(&Hook::PreLoad, &HookEvent::ShowEntry)?;
    let passphrase = utilities::get_passphrase("Enter passphrase: ", no_keyring)?;
    let storage = load_entries(&passphrase)?;
    if storage.entries.contains_key(entry) {
        let password = &storage
            .entries
            .get(entry)
            .ok_or_else(|| anyhow!("entry {} not found", entry))?
            .password;
        if on_screen {
            println!("{}", password);
        } else {
            utilities::copy_to_clipboard(password.to_string())?;
        }
    } else {
        return Err(anyhow!("{} not found", entry));
    }

    Ok(())
}

pub fn edit(entry: &str, no_keyring: bool) -> Result<()> {
    run_hook(&Hook::PreLoad, &HookEvent::ShowEntry)?;
    let passphrase = utilities::get_passphrase("Enter passphrase: ", no_keyring)?;
    let mut storage = load_entries(&passphrase)?;
    if storage.entries.contains_key(entry) {
        let password = rpassword::prompt_password_stdout(&format!("New password for {}: ", entry))?;
        storage.entries.insert(entry.to_owned(), Entry { password });
        save_entries(passphrase, &storage)?;
        run_hook(&Hook::PostSave, &HookEvent::EditEntry)?;
    } else {
        return Err(anyhow!("entry not found: {}", entry));
    };

    Ok(())
}

pub fn remove(entry: &str, no_keyring: bool) -> Result<()> {
    run_hook(&Hook::PreLoad, &HookEvent::ShowEntry)?;
    let passphrase = utilities::get_passphrase("Enter passphrase: ", no_keyring)?;
    let mut storage = load_entries(&passphrase)?;
    if storage.entries.remove(entry).is_some() {
        save_entries(passphrase, &storage)?;
        run_hook(&Hook::PostSave, &HookEvent::RemoveEntry)?;
    } else {
        return Err(anyhow!("entry not found: {}", entry));
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
        anyhow!("Failed to access password in keyring");
    }
    println!("Keyring integration seems fine");
    Ok(())
}

pub fn keyring_forget() -> Result<()> {
    let username = &whoami::username();
    let keyring = utilities::new_keyring(username);
    if keyring.delete_password().is_err() {
        anyhow!("Failed to delete password from keyring");
    }
    Ok(())
}
