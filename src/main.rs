use std::ffi::{c_void, OsString};
use std::path::PathBuf;
use std::slice;

use anyhow::{Context, Result};
use clap::Parser;
use windows::core::GUID;
use windows::Win32::Foundation::{HANDLE, PWSTR};
use windows::Win32::Globalization::lstrlenW;
use windows::Win32::UI::Shell;
use windows::Win32::UI::Shell::{SHGetKnownFolderPath, KF_FLAG_DEFAULT};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// If omitted, installs a shortcut to itself in Explorer's "Send To" menu.
    /// If passed, installs a shortcut to the .exe in the start menu.
    exe: Option<PathBuf>,
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

    if let Some(exe) = cli.exe {
        let start_menu = get_path(Shell::FOLDERID_StartMenu)?;
        // let exe_stem = exe
        //     .file_stem()
        //     .with_context(|| format!("Cannot identify filename of path {}", exe.display()))?;

        let mut shortcut = start_menu;

        // See https://users.rust-lang.org/t/append-an-additional-extension/23586
        // shortcut.push(exe_stem.to_owned() + OsStr::new(".lnk"));
        shortcut.push(exe.with_extension("lnk"));

        let abs_exe = if exe.is_absolute() {
            exe
        } else {
            let mut abs_exe = std::env::current_dir()
                .context("Cannot find current directory for relative path")?;
            abs_exe.push(exe);
            abs_exe
        };

        eprintln!(
            "create shortcut at \"{}\" to \"{}\"",
            shortcut.display(),
            abs_exe.display(),
        );
    } else {
        let send_to = get_path(Shell::FOLDERID_SendTo)?;

        eprintln!("install to \"{}\"", send_to.display());
    }

    Ok(())
}
