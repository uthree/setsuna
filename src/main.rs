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
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termsize;

fn main() {
    let (mut rows, mut cols) = (80, 24);
    let stdin = stdin();

    let mut stdout = MouseTerminal::from(
        stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap(),
    );

    let mut editor = TextEditor::new();
    editor.load_file(OsStr::new("./src/main.rs"));
    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", "\x1b[0m").unwrap();
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

    termsize::get().map(|size| (rows, cols) = (size.rows as usize, size.cols as usize));
    editor
        .render(Vector2::<usize> { x: cols, y: rows })
        .iter()
        .for_each(|l| write!(stdout, "{}", l).unwrap());

    stdout.flush().unwrap();

    for event in stdin.events() {
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", "\x1b[0m").unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        let event = event.unwrap();
        if event.clone() == Event::Key(Key::Char('q')) {
            return;
        }

        termsize::get().map(|size| (rows, cols) = (size.rows as usize, size.cols as usize));
        editor.recieve_event(event);
        editor
            .render(Vector2::<usize> { x: cols, y: rows })
            .iter()
            .for_each(|l| write!(stdout, "{}", l).unwrap());
        stdout.flush().unwrap();
    }
}
