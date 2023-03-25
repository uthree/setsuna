use crate::setsuna::core::vector2::Vector2;
use crate::setsuna::ui::block::RenderBlockResizable;
use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termsize;

pub trait Window {
    fn recieve_event(&mut self, event: Event);
    fn join(&mut self)
    where
        Self: RenderBlockResizable,
    {
        let mut stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        //get terminal size
        let mut rows = 24;
        let mut cols = 80;
        termsize::get().map(|size| {
            rows = size.rows;
            cols = size.cols;
        });
        // clear screen
        write!(stdout, "{}", clear::All).unwrap();
        // move cursor
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        // render
        self.render(Vector2 {
            x: cols as usize,
            y: rows as usize,
        })
        .iter()
        .for_each(|l| {
            write!(stdout, "{}", l);
        });
        stdout.flush().unwrap();

        for event in stdin.events() {
            //get terminal size
            let mut rows = 24;
            let mut cols = 80;
            termsize::get().map(|size| {
                rows = size.rows;
                cols = size.cols;
            });
            // clear screen
            write!(stdout, "{}", clear::All).unwrap();
            // move cursor
            write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
            // render
            self.render(Vector2 {
                x: cols as usize,
                y: rows as usize,
            })
            .iter()
            .for_each(|l| {
                write!(stdout, "{}", l).unwrap();
            });
            stdout.flush().unwrap();

            if event.unwrap() == Event::Key(Key::Ctrl('z')) {
                return;
            }
        }
    }
}
