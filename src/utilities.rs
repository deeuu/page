use anyhow::{anyhow, Error, Result};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use keyring::Keyring;
use secrecy::Secret;
use std::io;
use std::io::{Read, Write};

pub fn encrypt(plaintext: &[u8], passphrase: Secret<String>) -> Result<Vec<u8>, Error> {
    let encryptor = age::Encryptor::with_user_passphrase(passphrase);

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted).map_err(Error::msg)?;
    writer.write_all(plaintext)?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt(encrypted: &[u8], passphrase: &Secret<String>) -> Result<Vec<u8>, Error> {
    let decryptor = match age::Decryptor::new(&encrypted[..])? {
        age::Decryptor::Passphrase(d) => d,
        age::Decryptor::Recipients(..) => unreachable!(),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(passphrase, None)?;
    loop {
        let bytes = reader.read_to_end(&mut decrypted)?;
        if bytes == 0 {
            break;
        }
    }

    Ok(decrypted)
}

const KEYRING_APP_NAME: &str = "passage";

pub fn new_keyring(username: &str) -> Keyring {
    return Keyring::new(KEYRING_APP_NAME, username);
}

/// Gets the passphrase from either the keyring or stdin (and stores it in the keyring)
pub fn get_passphrase_keyring(prompt: &str) -> Result<Secret<String>> {
    let username = &whoami::username();
    let keyring = new_keyring(username);

    let passphrase = if let Ok(pw) = keyring.get_password() {
        Secret::new(pw)
    } else {
        let passphrase = rpassword::prompt_password_stdout(prompt)?;
        if keyring.set_password(&passphrase).is_err() {
            anyhow!("Failed to store password in keyring");
        }

        Secret::new(passphrase)
    };

    Ok(passphrase)
}

pub fn get_passphrase(prompt: &str, no_keyring: bool) -> Result<Secret<String>> {
    if no_keyring {
        let passphrase = rpassword::prompt_password_stdout(prompt)?;
        Ok(Secret::new(passphrase))
    } else {
        get_passphrase_keyring(prompt)
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
        copy_to_clipboard(attribute.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
use fork::{fork, Fork};

#[cfg(target_os = "linux")]
fn copy_to_clipboard(decrypted: String) -> Result<(), Error> {
    match fork() {
        Ok(Fork::Child) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new()
                .map_err(|e| anyhow!("failed to initialize clipboard provider: {}", e))?;
            ctx.set_contents(decrypted)
                .map_err(|e| anyhow!("failed to copy to clipboard: {}", e))?;

            std::thread::sleep(std::time::Duration::from_secs(10));

            ctx.set_contents("".to_owned())
                .map_err(|e| anyhow!("failed to copy to clipboard: {}", e))?;
        }
        Err(_) => return Err(Error::msg("Failed to fork()")),
        Ok(_) => {}
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn copy_to_clipboard(decrypted: String) -> Result<(), Error> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| anyhow!("failed to initialize clipboard provider: {}", e))?;
    ctx.set_contents(decrypted)
        .map_err(|e| anyhow!("failed to copy to clipboard: {}", e))?;
    Ok(())
}
