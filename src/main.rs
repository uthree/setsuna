mod setsuna;

use crate::setsuna::{
    core::vector2::Vector2,
    editor::text_editor::TextEditor,
    ui::{bar::Bar, block::RenderBlockResizable, window::Window},
};

fn main() {
    let mut editor = TextEditor::new();
    editor.load_file("./src/main.rs".into());
    editor.join();
}
