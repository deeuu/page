use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,

    #[arg(short, long)]
    /// Disable the keyring integration
    pub no_keyring: bool,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Initialize the password store
    Init,
    /// Add a new entry
    New {
        entry_name: String,

        #[arg(long, short)]
        username: Option<String>,

        #[arg(long)]
        url: Option<String>,
    },
    /// List all known entries
    List,
    /// Decrypt and show an entry
    Show {
        entry_name: String,

        // show all fields associated with this entry
        #[arg(long, short, value_enum, default_value_t = EntryAttribute::Password)]
        attribute: EntryAttribute,

        #[arg(long, short)]
        /// Print instead of copying it to the clipboard
        on_screen: bool,
    },
    /// Edit an entry
    Edit {
        entry_name: String,

        #[arg(long, short)]
        new_name: Option<String>,

        #[arg(long, short)]
        username: Option<String>,

        #[arg(long)]
        url: Option<String>,

        #[arg(long)]
        /// Do not display a prompt for entering a new password
        no_prompt: bool,
    },
    /// Remove an entry
    Remove { entry: String },
    /// Display status information
    Info,
    /// Keyring related commands
    Keyring {
        #[command(subcommand)]
        cmd: KeyringCmd,
    },
    /// Import entries from another password store
    Import {
        #[command(subcommand)]
        cmd: ImportCmd,
    },
}

#[derive(ValueEnum, Clone)]
pub enum EntryAttribute {
    Password,
    Username,
    Url,
}

#[derive(Subcommand)]
pub enum KeyringCmd {
    /// Checks if the keyring integration works
    Check,
    /// Deletes the password from the keyring
    Forget,
}

#[derive(Subcommand)]
pub enum ImportCmd {
    /// Import from keepass
    Keepass {
        database: PathBuf,
        keyfile: Option<PathBuf>,
        #[arg(long = "pw")]
        password: bool,
        #[arg(long, short)]
        prefix: Option<String>,
    },
}
