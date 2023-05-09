use crate::paths::entries_file;
use crate::utilities::{decrypt, encrypt};
use anyhow::{anyhow, Result};
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    fs::File,
    io::{BufReader, Read, Write},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    #[serde(flatten)]
    pub entries: HashMap<String, Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub password: String,
}

pub fn load_entries(passphrase: &Secret<String>) -> Result<Storage> {
    let mut encrypted: Vec<u8> = vec![];
    let file = match fs::metadata(entries_file()?) {
        Ok(_) => File::open(entries_file()?)?,
        Err(_) => return Err(anyhow!("storage not initialized, run `passage init`")),
    };
    let mut buf = BufReader::new(file);
    buf.read_to_end(&mut encrypted)?;
    if let 0 = encrypted.len() {
        Ok(Storage {
            entries: HashMap::new(),
        })
    } else {
        let decrypted = decrypt(&encrypted, passphrase)?;
        let decrypted = String::from_utf8(decrypted)?;
        let decrypted: Storage = toml::from_str(&decrypted)?;
        Ok(decrypted)
    }
}

pub fn save_entries(passphrase: Secret<String>, storage: &Storage) -> Result<()> {
    let bytes: Vec<u8> = toml::to_vec(&storage)?;
    let encrypted = encrypt(&bytes, passphrase)?;
    let mut file = File::create(entries_file()?)?;
    file.write_all(&encrypted)?;
    Ok(())
}
