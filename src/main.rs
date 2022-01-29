use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a "Send to start" item in the "Send to" menu.
    Install,

    /// Create a Start Menu shortcut from a .exe file.
    Create {
        exe: PathBuf,
    },

    /// Remove the "Send to start" item from the "Send to" menu.
    Uninstall,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Some(Commands::Install) => {
            eprintln!("install to \"Send to\"");
        }
        Some(Commands::Create {ref exe}) => {
            eprintln!("create shortcut to {}", exe.display());
        }
        Some(Commands::Uninstall) => {
            eprintln!("uninstall");
        }
        None => {
            eprintln!("Install \"Send to start\"?");
        }
    }

    // Continued program logic goes here...
}
