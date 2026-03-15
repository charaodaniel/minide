mod document;
mod editor;
mod row;
mod terminal;

use editor::Editor;

fn main() {
    Editor::default().run();
}
