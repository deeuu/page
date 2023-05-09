use clap::{Parser, Subcommand};

#[derive(Parser)]
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
    New,
    /// List all known entries
    List,
    /// Decrypt and show an entry
    Show {
        entry: String,

        #[arg(long, short)]
        /// Print the password instead of copying it to the clipboard
        on_screen: bool,
    },
    /// Edit an entry
    Edit { entry: String },
    /// Remove an entry
    Remove { entry: String },
    /// Display status information
    Info,
    /// Keyring related commands
    Keyring {
        #[command(subcommand)]
        cmd: KeyringCmd,
    },
}

#[derive(Subcommand)]
pub enum KeyringCmd {
    /// Checks if the keyring integration works
    Check,
    /// Deletes the password from the keyring
    Forget,
}
