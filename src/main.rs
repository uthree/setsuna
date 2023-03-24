mod setsuna;

use crate::setsuna::{
    core::vector2::Vector2,
    editor::text_editor::TextEditor,
    ui::{bar::Bar, block::RenderBlockResizable},
};

fn main() {
    let mut editor = TextEditor::new();
    editor.load_file("./src/main.rs".into());
    editor
        .render(Vector2 { x: 80, y: 24 })
        .iter()
        .for_each(|s| print!("{}", s));
}
