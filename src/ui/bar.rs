use crate::core::text::Text;
use crate::core::text::TextStyle;
use crate::render::color::remove_color_settings;
use crate::render::renderable::RenderableLine;
use colored::Color;

pub enum Pivot {
    Left,
    Center,
    Right,
}

pub enum BarContent {
    Text(Text),
}

pub struct Bar {
    content: BarContent,
    style: TextStyle,
    size: u16,
    left: String,
    right: String,
    pivot: Pivot,
    padding_char: char,
}

impl Bar {
    pub fn label(text: &str) -> Self {
        Bar {
            content: BarContent::Text(Text::new(text)),
            size: remove_color_settings(text.to_string()).len() as u16,
            style: TextStyle::default(),
            left: "[".to_string(),
            right: "]".to_string(),
            pivot: Pivot::Left,
            padding_char: '-',
        }
    }
}

impl RenderableLine for Bar {
    fn render(&self) -> String {
        match &self.content {
            BarContent::Text(t) => {
                let mut content_len = t.size() as usize;
                let mut buff_lr = String::from("");
                let l_len = remove_color_settings(self.left.clone()).len();
                let r_len = remove_color_settings(self.right.clone()).len();
                let mut render_content = t.render();

                // no overflow
                let available_len = self.size - (l_len + r_len) as u16;
                if (available_len as usize) < content_len {
                    render_content = "".to_string();
                    content_len = 0;
                }
                let pad_len = self.size as usize - content_len - l_len - r_len;
                for _ in 0..(pad_len) {
                    buff_lr.push(self.padding_char)
                }
                match &self.pivot {
                    Pivot::Left => vec![
                        self.left.clone(),
                        render_content,
                        buff_lr,
                        self.right.clone(),
                    ]
                    .join(""),
                    Pivot::Right => vec![
                        self.left.clone(),
                        buff_lr,
                        render_content,
                        self.right.clone(),
                    ]
                    .join(""),
                    Pivot::Center => {
                        let pad_l = pad_len / 2;
                        let pad_r = pad_len - pad_l;
                        let mut buff = self.left.clone();
                        for _ in 0..pad_l {
                            buff.push(self.padding_char)
                        }
                        buff = vec![buff, render_content].join("");
                        for _ in 0..pad_r {
                            buff.push(self.padding_char)
                        }
                        vec![buff, self.right.clone()].join("")
                    }
                }
            }
        }
    }
    fn size(&self) -> u16 {
        match &self.content {
            BarContent::Text(_) => {
                std::cmp::max(self.size, self.left.len() as u16 + self.right.len() as u16)
            }
        }
    }
}
