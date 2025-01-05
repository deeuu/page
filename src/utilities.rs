use age::secrecy::SecretString;
use anyhow::{anyhow, Error, Result};
use arboard::Clipboard;
use keyring::Keyring;
use std::io;
use std::io::{Read, Write};

pub fn encrypt(plaintext: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_user_passphrase(passphrase);

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted).map_err(Error::msg)?;
    writer.write_all(plaintext)?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt(encrypted: &[u8], passphrase: SecretString) -> Result<Vec<u8>, Error> {
    let identity = age::scrypt::Identity::new(passphrase);
    let decryptor = age::Decryptor::new_buffered(encrypted)?;
    let mut reader = decryptor.decrypt(Some(&identity as _).into_iter())?;
    let mut decrypted = vec![];
    loop {
        let bytes = reader.read_to_end(&mut decrypted)?;
        if bytes == 0 {
            break;
        }
    }
    Ok(decrypted)
}

const KEYRING_APP_NAME: &str = "page";

pub fn new_keyring(username: &str) -> Keyring {
    Keyring::new(KEYRING_APP_NAME, username)
}

/// Gets the passphrase from either the keyring or stdin (and stores it in the keyring)
pub fn get_passphrase_keyring(prompt: &str) -> Result<SecretString> {
    let username = &whoami::username();
    let keyring = new_keyring(username);

    let passphrase = if let Ok(pw) = keyring.get_password() {
        SecretString::from(pw)
    } else {
        let passphrase = rpassword::prompt_password_stdout(prompt)?;
        if keyring.set_password(&passphrase).is_err() {
            return Err(anyhow!("Failed to store password in keyring"));
        }

        SecretString::from(passphrase)
    };

    Ok(passphrase)
}

pub fn get_passphrase(no_keyring: bool) -> Result<SecretString> {
    const PROMPT: &str = "Enter passphrase: ";
    if no_keyring {
        let passphrase = rpassword::prompt_password_stdout(PROMPT)?;
        Ok(SecretString::from(passphrase))
    } else {
        get_passphrase_keyring(PROMPT)
    }
}

pub fn read_stdin(msg: &str) -> Result<String> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut entry = String::new();
    io::stdin().read_line(&mut entry)?;
    let entry = entry.trim();
    Ok(entry.to_owned())
}

pub fn reveal(attribute: &String, on_screen: bool) -> Result<()> {
    if on_screen {
        println!("{}", attribute);
    } else {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(attribute.to_string())?;
    }
    Ok(())
}
