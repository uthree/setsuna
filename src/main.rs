pub mod setsuna;

use crate::setsuna::core::vector2::Vector2;
use crate::setsuna::editor::text_editor::TextEditor;
use crate::setsuna::ui::block::RenderableBlockResizable;
use crate::setsuna::ui::window::Window;
use std::ffi::OsStr;
use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;

fn main() {
    let stdin = stdin();

    let mut stdout = stdout().into_alternate_screen().unwrap();

    let mut editor = TextEditor::new();
    editor.load_file(OsStr::new("./src/main.rs"));
    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", "\x1b[0m").unwrap();
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
    editor
        .render(Vector2::<usize> { x: 80, y: 24 })
        .iter()
        .for_each(|l| write!(stdout, "{}", l).unwrap());

    stdout.flush().unwrap();

    for event in stdin.events() {
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", "\x1b[0m").unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        let event = event.unwrap();
        editor.recieve_event(event);
        editor
            .render(Vector2::<usize> { x: 80, y: 24 })
            .iter()
            .for_each(|l| write!(stdout, "{}", l).unwrap());
        stdout.flush().unwrap();
    }
}
