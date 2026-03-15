mod app;
mod editor;
mod explorer;
mod syntax;
mod ui;

use app::App;

fn main() {
    if let Err(e) = App::new().run() {
        eprintln!("Error: {}\n", e);
    }
}
