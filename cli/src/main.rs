use core::editor::Editor;

fn main() {
    let path = std::env::args().nth(1);
    Editor::new(path.as_deref()).run();
}
