#![windows_subsystem = "windows"]

mod editor;
mod ui;
use fltk::app;
use ui::*;

fn main() {
    let a = app::App::default();
    build_ui();
    a.run().unwrap();
}
