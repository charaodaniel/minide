// main.rs
mod app;
mod editor;
mod explorer;
mod git;
mod syntax;
mod ui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    // Test change for Git integration
    let mut app = App::new();
    app.run()
}
