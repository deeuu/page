mod cli;
mod entries;
mod hooks;
mod paths;
mod utilities;
use anyhow::Result;
pub use clap::Parser;
mod commands;
use cli::{Cli, Cmd, KeyringCmd};

fn main() -> Result<()> {
    let opt = Cli::parse();

    match opt.cmd {
        Cmd::Init => commands::init(opt.no_keyring),
        Cmd::New {
            entry_name,
            username,
            url,
        } => commands::new_entry(entry_name, username, url, opt.no_keyring),
        Cmd::List => commands::list(opt.no_keyring),
        Cmd::Show {
            entry_name,
            attribute,
            on_screen,
        } => commands::show(&entry_name, attribute, on_screen, opt.no_keyring),
        Cmd::Edit {
            entry_name,
            new_name,
            username,
            url,
            no_prompt,
        } => commands::edit(
            entry_name,
            new_name,
            username,
            url,
            no_prompt,
            opt.no_keyring,
        ),
        Cmd::Remove { entry } => commands::remove(&entry, opt.no_keyring),
        Cmd::Info => commands::info(),
        Cmd::Keyring { cmd } => match cmd {
            KeyringCmd::Check => commands::keyring_check(),
            KeyringCmd::Forget => commands::keyring_forget(),
        },

        Cmd::Completions { shell } => {
            commands::shell_completion(shell);
            Ok(())
        }
    }
}
