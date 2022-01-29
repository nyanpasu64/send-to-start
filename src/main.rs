use std::env::current_exe;
use std::ffi::{c_void, OsString};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::slice;

use anyhow::{Context, Result};
use clap::Parser;
use windows::core::{Interface, GUID};
use windows::Win32::Foundation::{HANDLE, PWSTR};
use windows::Win32::Globalization::lstrlenW;
use windows::Win32::System::Com;
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoTaskMemFree, IPersistFile};
use windows::Win32::UI::Shell;
use windows::Win32::UI::Shell::{IShellLinkW, SHGetKnownFolderPath, KF_FLAG_DEFAULT};

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
        let path_ptr: PWSTR = SHGetKnownFolderPath(&folder_id, KF_FLAG_DEFAULT as u32, HANDLE(0))?;
        let path_ref: &[u16] = slice::from_raw_parts(path_ptr.0, lstrlenW(path_ptr) as usize);
        let path_copy: PathBuf = OsString::from_wide(path_ref).into();
        CoTaskMemFree(path_ptr.0 as *mut c_void);
        Ok(path_copy)
    }
}

struct IconPath {
    path: PathBuf,
    index: i32,
}

fn create_lnk(link: &Path, target: &Path, icon: Option<&IconPath>) -> Result<()> {
    // Ported from https://stackoverflow.com/a/3907013.
    unsafe {
        let shell_link: IShellLinkW =
            CoCreateInstance(&Shell::ShellLink, None, Com::CLSCTX_INPROC_SERVER)
                .context("loading .lnk file library")?;
        shell_link
            .SetPath(target.as_os_str())
            .with_context(|| format!("setting .lnk path to \"{}\"", target.display()))?;

        // Pick the "link" icon.
        if let Some(icon) = &icon {
            shell_link
                .SetIconLocation(icon.path.as_os_str(), icon.index)
                .context("setting icon")?;
        }

        let persist_file: IPersistFile = shell_link
            .cast::<IPersistFile>()
            .context("loading .lnk saving interface")?;
        persist_file
            .Save(link.as_os_str(), true)
            .with_context(|| format!("writing .lnk file to \"{}\"", link.display()))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null(), Com::COINIT_APARTMENTTHREADED)
            .context("failed to initialize COM")?;
    }

    let cli = Cli::parse();

    if let Some(exe) = cli.exe {
        let start_menu =
            get_path(Shell::FOLDERID_StartMenu).context("failed to find Start menu path")?;

        let mut folder = start_menu;
        folder.push("Shortcuts");
        create_dir_all(&folder).with_context(|| {
            format!(
                "failed to create Shortcuts folder at \"{}\"",
                folder.display()
            )
        })?;

        let mut shortcut = folder;
        // See https://users.rust-lang.org/t/append-an-additional-extension/23586
        let lnk_name = {
            let exe_name = exe
                .file_name()
                .with_context(|| format!("cannot identify filename of path {}", exe.display()))?;
            Path::new(exe_name).with_extension("lnk")
        };
        shortcut.push(lnk_name);

        let abs_exe = if exe.is_absolute() {
            exe
        } else {
            let mut abs_exe =
                std::env::current_dir().context("failed to resolve relative EXE path")?;
            abs_exe.push(exe);
            abs_exe
        };

        eprintln!(
            "create shortcut at \"{}\" to \"{}\"",
            shortcut.display(),
            abs_exe.display(),
        );
        create_lnk(&shortcut, &abs_exe, None).context("failed to create app shortcut")?;

        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;
    } else {
        let send_to = get_path(Shell::FOLDERID_SendTo).context("failed to find Send To path")?;
        let mut shortcut = send_to;
        shortcut.push("Start (create shortcut).lnk");

        let exe_name = current_exe().context("failed to locate program for Send To")?;

        // imageres.dll (163) is the link icon.
        let mut icon_path =
            get_path(Shell::FOLDERID_System).context("obtaining system32 folder for icon")?;
        icon_path.push("imageres.dll");
        let icon_path = IconPath {
            path: icon_path,
            index: 154,
        };

        eprintln!(
            "install shortcut at \"{}\" to \"{}\"",
            shortcut.display(),
            exe_name.display()
        );
        create_lnk(&shortcut, &exe_name, Some(&icon_path)).context("failed to install to Send To")?;
    }

    Ok(())
}
