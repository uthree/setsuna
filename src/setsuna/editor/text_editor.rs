use crate::setsuna::{
    core::vector2::Vector2,
    ui::{
        bar::Bar, block::RenderableBlockResizable, line::RenderLineResizable,
        text_buffer::TextBuffer,
    },
};
use colored::Color;
use std::ffi::OsStr;
use std::fs;

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
    pub logical_cursor_position: Vector2<usize>,
    pub mode: Mode,
}

impl RenderableBlockResizable for TextEditor {
    fn render(&self, size: Vector2<usize>) -> Vec<String> {
        let mut buffer_renderer = TextBuffer::from_buffer(&self.buffer);

        // build bar
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

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            buffer: vec![],
            logical_cursor_position: Vector2::<usize> { x: 0, y: 0 },
            mode: Mode::Normal,
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
