use crate::setsuna::{
    core::vector2::Vector2,
    ui::{
        bar::Bar, block::RenderBlockResizable, line::RenderLineResizable, text::TextStyle,
        window::Window,
    },
};
use colored::Color;
use ropey::Rope;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use termion::event::{Event, Key};

#[derive(PartialEq, Clone)]
pub enum Mode {
    Normal,
    Insert,
}

impl Mode {
    pub fn to_str(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
        }
    }
}

pub struct TextEditor {
    pub begin_of_render: usize,
    pub buffer: Rope,
    pub number_style: TextStyle,
    pub mode: Mode,
    pub cursor: Vector2<usize>,
}

impl TextEditor {
    pub fn new() -> Self {
        let mut s = TextStyle::default();
        s.foreground_color = Some(Color::Black);
        TextEditor {
            begin_of_render: 0,
            buffer: Rope::new(),
            number_style: s,
            mode: Mode::Normal,
            cursor: Vector2 { x: 0, y: 0 },
        }
    }

    pub fn load_file(&mut self, path: OsString) -> Result<(), io::Error> {
        self.buffer = Rope::from_reader(BufReader::new(File::open(path)?))?;
        Ok(())
    }
}

impl Window for TextEditor {
    fn recieve_event(&mut self, event: Event) {
        if self.mode != Mode::Normal {
            if event == Event::Key(Key::Esc) {
                self.mode = Mode::Normal;
            }
        }
    }
}

impl RenderBlockResizable for TextEditor {
    fn render(&self, size: Vector2<usize>) -> Vec<String> {
        let mut render_buff = Vec::<String>::new();
        let mut row = self.begin_of_render;
        let num_digit_of_line_number = (self.buffer.len_lines() + 1).to_string().len();

        while row < size.y - 1 {
            match self.buffer.get_line(row) {
                Some(line) => {
                    let mut col = 0;
                    let d = (row + 1).to_string();
                    let d_pad_len = num_digit_of_line_number - d.len();
                    let d = [(0..d_pad_len).map(|_| ' ').collect::<String>(), d].join("");

                    let mut line_buff = self.number_style.apply(vec![d, " ".to_string()].join(""));
                    while col + num_digit_of_line_number + 1 < size.x {
                        // render line
                        if col + num_digit_of_line_number < line.to_string().len() {
                            line_buff.push(line.to_string().chars().nth(col).unwrap());
                        } else {
                            line_buff.push(' ');
                        }
                        col += 1;
                    }
                    render_buff.push(line_buff)
                }
                None => render_buff.push((0..size.x).map(|_| ' ').collect::<String>()),
            }
            row += 1;
        }

        // build status bar
        let bar_left = Bar::label(&format!(" {} ", self.mode.to_str()))
            .on_black()
            .left();
        let bar_right = Bar::label(" Unix | UTF-8 | Rust ").on_black().right();
        let bar_center = Bar::label(" main.rs ").on_black();

        let status_bar = Bar::bars(vec![bar_left, bar_center, bar_right]);

        render_buff.push(status_bar.render(size.x));
        return render_buff;
    }
}
