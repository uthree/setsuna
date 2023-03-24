use crate::setsuna::core::vector2::Vector2;
use crate::setsuna::ui::block::RenderableBlockResizable;
use crate::setsuna::ui::text::remove_special;
use crate::setsuna::ui::text::TextStyle;
use ropey::Rope;

pub struct TextBuffer<'a> {
    pub view_start_line: usize,
    pub show_line_number: bool,
    pub wrap: bool,
    pub buffer: &'a Rope,
    pub line_style: TextStyle,
}

impl TextBuffer<'_> {
    pub fn from_buffer<'a>(buffer: &'a Rope) -> TextBuffer<'a> {
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
        let buffer_len = self.buffer.len_chars();
        let num_line_number_digits = (buffer_len + 1).to_string().len();

        let mut render_buff = Vec::<String>::new();
        let mut now_line = self.view_start_line;

        while render_buff.len() < size.y {
            let mut line_buff = "".to_string();
            let line = self.buffer.get_line(now_line);
            match line {
                Some(l) => {
                    line_buff = l.to_string();
                }
                None => {}
            }

            //fill space
            //while line_buff.len() < size.x - 1 {
            //    line_buff.push(' ');
            //}
            // remove overflow text
            //line_buff = line_buff[..size.x].to_string();
            now_line = now_line + 1;

            render_buff.push(line_buff);
        }
        render_buff
    }
}
