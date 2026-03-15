mod app;
mod editor;
mod explorer;
mod ui;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
