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
    let opt = Cli::try_parse()?;

    match opt.cmd {
        Cmd::Init => commands::init(opt.no_keyring),
        Cmd::New => commands::new_entry(opt.no_keyring),
        Cmd::List => commands::list(opt.no_keyring),
        Cmd::Show { entry, on_screen } => commands::show(&entry, on_screen, opt.no_keyring),
        Cmd::Edit { entry } => commands::edit(&entry, opt.no_keyring),
        Cmd::Remove { entry } => commands::remove(&entry, opt.no_keyring),
        Cmd::Info => commands::info(),
        Cmd::Keyring { cmd } => match cmd {
            KeyringCmd::Check => commands::keyring_check(),
            KeyringCmd::Forget => commands::keyring_forget(),
        },
    }
}
