mod app;
mod editor;
mod explorer;
mod git;
mod syntax;
mod ui;

use app::App;

fn main() {
    if let Err(e) = App::new().run() {
        eprintln!("Error: {}\n", e);
    }
}
