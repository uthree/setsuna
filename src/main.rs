mod core;
mod editor;
mod ui;

use crate::core::vector2::Vector2;
use crate::core::*;
use crate::ui::block::Block;
use crate::ui::*;

use crate::editor::text_editor::TextEditor;

fn main() {
    let mut editor = TextEditor::default();
    let lines = editor.render(Vector2 { x: 80, y: 24 });
    for line in lines {
        println!("{}", line);
    }
}
