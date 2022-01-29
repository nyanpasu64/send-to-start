# Send to Start

A tool to add programs to Windows Start menu search without pinning them

## Install

Download send-to-start.exe from [Releases](https://github.com/nyanpasu64/send-to-start/releases), move it to a stable location (like an apps folder), then double-click it. This will install a "Start (create shortcut)" item in File Explorer's "Send to" menu, linking to the .exe's path.

If you move send-to-start.exe to a different path, run it again to fix the "Send to" item.

To uninstall the app (delete the "Send to" item), open File Explorer to `%APPDATA%\Microsoft\Windows\SendTo` and delete "Start (create shortcut)". You can delete send-to-start.exe as well.

## Usage

Open File Explorer and right-click a .exe file. In the menu, expand the "Send to" menu (you can press the `n` key), then select "Start (create shortcut)".

This creates a new shortcut in the folder `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Shortcuts`.

<!-- ## Thanks

TODO -->

## Contributing

Bug reports and feedback are encouraged; use [Issues](https://github.com/nyanpasu64/send-to-start/issues) for bugs and [Discussions](https://github.com/nyanpasu64/send-to-start/discussions) for questions and comments. PRs are welcome, but will be accepted at my discretion.

## License

Licensed under the [Mozilla Public License 2.0](LICENSE), by [nyanpasu64](https://github.com/nyanpasu64).
