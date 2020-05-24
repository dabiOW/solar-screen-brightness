#![windows_subsystem = "windows"]

mod assets;
mod tray;
mod wide;
mod console;
mod tui;

use crate::tray::TrayApplication;

fn main() {
    let app = TrayApplication::create();
    app.run();
}
