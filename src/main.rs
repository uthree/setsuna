use clap::{App, Arg};
use std::io::Write;
use std::io::{stdin, stdout, Stdout};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
mod editor;
use editor::{Editor, Mode};
use std::ffi::OsStr;

fn main() {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut height: u16 = 32;
    let mut width: u16 = 32;
    if let Some((w, h)) = term_size::dimensions() {
        width = w as u16;
        height = h as u16;
    }

    //clap
    let matches = App::new("setsuna")
        .about("A text editor")
        .bin_name("setsuna")
        .arg(Arg::with_name("file"))
        .get_matches();

    let mut editor = Editor::new();

    // get path
    let file_path: Option<&OsStr> = matches.value_of_os("file");
    // load file
    if let Some(_) = file_path {
        editor.load_file(file_path.unwrap());
    }

    clear_screen(&mut stdout);
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
    let rendered = editor.render(height.into(), width.into());
    let rendered = convert_buffer_to_string(rendered);
    write!(stdout, "{}", rendered).unwrap();
    write!(
        stdout,
        "{}",
        cursor::Goto(editor.cursor.x + 1, editor.cursor.y + 1)
    )
    .unwrap();

    stdout.flush().unwrap();

    for evt in stdin.events() {
        if let Some((w, h)) = term_size::dimensions() {
            width = w as u16;
            height = h as u16;
        }
        let e = evt.unwrap();
        match e {
            //terminate
            Event::Key(Key::Ctrl('c')) => {
                return;
            }

            _ => {
                if e == Event::Key(Key::Right) {
                    editor.cursor.x += 1;
                }
                if e == Event::Key(Key::Left) {
                    if editor.cursor.x > 0 {
                        editor.cursor.x -= 1;
                    }
                }
                if e == Event::Key(Key::Down) {
                    if height > editor.cursor.y + 3 {
                        editor.cursor.y += 1;
                    } else {
                        editor.screen_begin_row += 1;
                    }
                }
                if e == Event::Key(Key::Up) {
                    if editor.cursor.y > 0 {
                        editor.cursor.y -= 1;
                    } else if editor.screen_begin_row > 0 {
                        editor.screen_begin_row -= 1;
                    }
                }
                if e == Event::Key(Key::Char(':')) || editor.mode == Mode::Normal {
                    editor.mode = Mode::Command;
                }
            }
        }
        clear_screen(&mut stdout);
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        let rendered = editor.render(height.into(), width.into());
        let rendered = convert_buffer_to_string(rendered);
        write!(stdout, "{}", rendered).unwrap();

        write!(
            stdout,
            "{}",
            cursor::Goto(editor.cursor.x + 1, editor.cursor.y + 1)
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}

fn clear_screen(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}", clear::All);
}

fn convert_buffer_to_string(buffer: Vec<Vec<char>>) -> String {
    let mut ret = String::new();
    for line in buffer {
        for c in line {
            ret.push(c);
        }
        //ret.push('\n')
    }
    ret
}
