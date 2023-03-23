use crate::setsuna::core::vector2::Vector2;
use crate::setsuna::ui::block::RenderableBlockResizable;
use crate::setsuna::ui::text::remove_special;
use crate::setsuna::ui::text::TextStyle;

pub struct TextBuffer<'a> {
    pub view_start_line: usize,
    pub show_line_number: bool,
    pub wrap: bool,
    pub buffer: &'a Vec<String>,
    pub line_style: TextStyle,
}

impl TextBuffer<'_> {
    pub fn from_buffer<'a>(buffer: &'a Vec<String>) -> TextBuffer<'a> {
        TextBuffer {
            view_start_line: 0,
            show_line_number: true,
            wrap: false,
            buffer: buffer,
            line_style: TextStyle::default(),
        }
    }
}

impl RenderableBlockResizable for TextBuffer<'_> {
    fn render(&self, size: Vector2<usize>) -> Vec<String> {
        let buffer_len = self.buffer.len();
        let num_line_number_digits = (buffer_len + 1).to_string().len();

        let mut render_buff = Vec::<String>::new();
        let mut line = self.view_start_line;

        while render_buff.len() < size.y {
            let mut line_buff = "".to_string();

            // render line number
            let mut number = "".to_string();
            if self.show_line_number {
                let spaces = (0..(num_line_number_digits + 1 - (line + 1).to_string().len()))
                    .map(|_| ' ')
                    .collect::<String>();
                number = vec![spaces, (line + 1).to_string(), " ".to_string()].join("");
                if line > self.buffer.len() - 2 {
                    number = (0..(num_line_number_digits + 1))
                        .map(|_| ' ')
                        .collect::<String>();
                }
            }
            line_buff = number;

            // render text
            if self.buffer.len() > line + 1 {
                line_buff = vec![line_buff, self.buffer[line].clone()].join("");
            }

            // fill space
            while line_buff.len() < size.x {
                line_buff = vec![line_buff, " ".to_string()].join("");
            }

            // crop overflowed text
            line_buff = line_buff[0..(size.x)].to_string();

            line = line + 1;
            render_buff.push(line_buff);
        }
        render_buff
    }
}
