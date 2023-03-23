pub mod setsuna;

use crate::setsuna::core::vector2::Vector2;
use crate::setsuna::editor::text_editor::TextEditor;
use crate::setsuna::ui::bar::*;
use crate::setsuna::ui::block::RenderableBlockResizable;
use crate::setsuna::ui::line::{Pivot, RenderLineResizable};
use crate::setsuna::ui::text::Text;
use colored::Color;
use colored::Colorize;
use std::ffi::OsStr;

fn main() {
    let mut editor = TextEditor::new();
    editor.load_file(OsStr::new("./src/setsuna/core/vector2.rs"));
    editor
        .render(Vector2::<usize> { x: 80, y: 24 })
        .iter()
        .for_each(|l| println!("{}", l))
}
