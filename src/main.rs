use ::core::ffi::c_void;
use std::ffi::OsString;
use std::path::PathBuf;
use std::slice;

use anyhow::Context;
use anyhow::Result;
use windows::core::GUID;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::PWSTR;
use windows::Win32::Globalization::lstrlenW;
use windows::Win32::UI::Shell;
use windows::Win32::UI::Shell::{SHGetKnownFolderPath, KF_FLAG_DEFAULT};

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
    Create { exe: PathBuf },

    /// Remove the "Send to start" item from the "Send to" menu.
    Uninstall,
}

fn get_path(folder_id: GUID) -> Result<PathBuf> {
    use std::os::windows::prelude::OsStringExt;
    unsafe {
        let path_ptr: PWSTR =
            SHGetKnownFolderPath(&folder_id, KF_FLAG_DEFAULT as u32, HANDLE(0))
                .with_context(|| format!("Cannot find start menu path for {:?}", folder_id))?;
        let path_ref: &[u16] = slice::from_raw_parts(path_ptr.0, lstrlenW(path_ptr) as usize);
        let path_copy: PathBuf = OsString::from_wide(path_ref).into();
        windows::Win32::System::Com::CoTaskMemFree(path_ptr.0 as *mut c_void);
        Ok(path_copy)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Some(Commands::Install) => {
            let send_to = get_path(Shell::FOLDERID_SendTo)?;

            eprintln!("install to \"{}\"", send_to.display());
        }
        Some(Commands::Create { ref exe }) => {
            let start_menu = get_path(Shell::FOLDERID_StartMenu)?;
            eprintln!(
                "create shortcut at \"{}\" to \"{}\"",
                start_menu.display(),
                exe.display(),
            );
        }
        Some(Commands::Uninstall) => {
            eprintln!("uninstall");
        }
        None => {
            eprintln!("Install \"Send to start\"?");
        }
    }

    // Continued program logic goes here...
    Ok(())
}
