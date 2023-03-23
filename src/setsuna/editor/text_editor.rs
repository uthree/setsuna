use crate::setsuna::{
    core::vector2::Vector2,
    ui::{
        bar::Bar, block::RenderableBlockResizable, line::RenderLineResizable, text::TextStyle,
        text_buffer::TextBuffer, window::Window,
    },
};
use colored::Color;
use std::ffi::OsStr;
use std::fs;
use termion::event::{Event, Key};

pub enum Mode {
    Normal,
    Insert,
}

impl Mode {
    fn to_str(&self) -> &str {
        match &self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
        }
    }
}

pub struct TextEditor {
    pub buffer: Vec<String>,
    pub view_start_line: usize,
    pub cursor: Vector2<usize>,
    pub mode: Mode,
    pub line_style: TextStyle,
}

impl RenderableBlockResizable for TextEditor {
    fn render(&self, size: Vector2<usize>) -> Vec<String> {
        let mut buffer_renderer = TextBuffer::from_buffer(&self.buffer);
        buffer_renderer.view_start_line = self.cursor.y;
        buffer_renderer.line_style = self.line_style.clone();

        // build status bar
        let mut bar_mode = Bar::label(&vec![" ", self.mode.to_str(), " "].join("")).left();
        let mut bar_filename = Bar::label("FileName");
        let mut bar_cursor_status = Bar::label(" unix | uft-8 ").right();
        let mut status_bar = Bar::bars(vec![bar_mode, bar_filename, bar_cursor_status]);
        status_bar.style.background_color = Some(Color::Black);

        let (siz_x, siz_y) = (size.x, size.y);
        let b_siz_y = siz_y - 1;
        let buff_render_size = Vector2::<usize> {
            x: siz_x,
            y: b_siz_y,
        };
        let mut rend_buff = buffer_renderer.render(buff_render_size);
        let rend_bar = status_bar.render(size.x);
        rend_buff.push(rend_bar);
        rend_buff
    }
}

impl Window for TextEditor {
    fn recieve_event(&mut self, event: Event) {
        // mode shifts
        match self.mode {
            Mode::Insert => {
                if event == Event::Key(Key::Esc) {
                    self.mode = Mode::Normal;
                };
            }
            Mode::Normal => {
                if event == Event::Key(Key::Char('a')) || event == Event::Key(Key::Char('i')) {
                    self.mode = Mode::Insert;
                };
            }
            _ => {}
        }

        // move cursor
        match self.mode {
            Mode::Insert | Mode::Normal => match event {
                Event::Key(Key::Down) => self.cursor.y += 1,
                Event::Key(Key::Up) => {
                    if self.cursor.y > 0 {
                        self.cursor.y -= 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl TextEditor {
    pub fn new() -> Self {
        let mut line_style = TextStyle::default();
        line_style.foreground_color = Some(Color::Red);
        TextEditor {
            buffer: vec![],
            cursor: Vector2::<usize> { x: 0, y: 0 },
            mode: Mode::Insert,
            view_start_line: 0,
            line_style: line_style,
        }
    }
    pub fn load_file(&mut self, path: &OsStr) {
        self.buffer = fs::read_to_string(path)
            .unwrap()
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
    }
}
