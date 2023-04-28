use crate::ui::line::Line;
use crate::ui::text::{Pivot, Text, TextStyle};
use colored::Color;
use regex::Regex;

fn remove_special_char(i: &String) -> String {
    let re = Regex::new(r"\x1b\x5b(.*)m").unwrap();
    re.replace_all(i, "").into()
}

pub enum BarContent {
    Text(Text),
    Bars(Vec<Bar>),
}

pub struct Bar {
    content: BarContent,
    pub left: String,
    pub right: String,
    pub style: TextStyle,
}

impl Bar {
    pub fn label(label: &str) -> Self {
        Bar {
            content: BarContent::Text(Text::label(label)),
            style: TextStyle::default(),
            left: "".to_string(),
            right: "".to_string(),
        }
    }

    pub fn bars(bars: Vec<Bar>) -> Self {
        Bar {
            content: BarContent::Bars(bars),
            style: TextStyle::default(),
            left: "".to_string(),
            right: "".to_string(),
        }
    }

    pub fn set_pivot(&mut self, pivot: Pivot) {
        match &mut self.content {
            BarContent::Text(text) => text.pivot = pivot,
            _ => {}
        }
    }

    pub fn left(mut self) -> Self {
        self.set_pivot(Pivot::Left);
        self
    }

    pub fn center(mut self) -> Self {
        self.set_pivot(Pivot::Center);
        self
    }

    pub fn right(mut self) -> Self {
        self.set_pivot(Pivot::Right);
        self
    }

    pub fn on_red(mut self) -> Self {
        self.style.background_color = Some(Color::Red);
        self
    }
    pub fn on_blue(mut self) -> Self {
        self.style.background_color = Some(Color::Blue);
        self
    }
    pub fn on_green(mut self) -> Self {
        self.style.background_color = Some(Color::Green);
        self
    }
    pub fn on_black(mut self) -> Self {
        self.style.background_color = Some(Color::Black);
        self
    }
    pub fn on_yellow(mut self) -> Self {
        self.style.background_color = Some(Color::Yellow);
        self
    }
    pub fn on_white(mut self) -> Self {
        self.style.background_color = Some(Color::White);
        self
    }
}

impl Line for Bar {
    fn render(&self, size: usize) -> String {
        if size == 0 {
            return "".to_string();
        };
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        match &self.content {
            BarContent::Text(text) => {
                if text.label.len() > size {
                    left = "".to_string();
                    right = "".to_string();
                }
            }
            _ => {}
        }
        match &self.content {
            BarContent::Text(text) => {
                // Render text
                let len_l = remove_special_char(&self.left).len();
                let len_r = remove_special_char(&self.right).len();

                vec![
                    "\x1b[0m".to_string(),
                    left,
                    self.style.get_begin(),
                    text.clone().render(size - len_l - len_r),
                    self.style.get_end(),
                    "\x1b[0m".to_string(),
                    right,
                    "\x1b[0m".to_string(),
                ]
                .join("")
            }
            BarContent::Bars(bars) => {
                let len_l = remove_special_char(&self.left).len();
                let len_r = remove_special_char(&self.right).len();
                let s = size - len_l - len_r;
                let sizes = (0..bars.len())
                    .map(|i| (s + i) / bars.len())
                    .collect::<Vec<usize>>();
                vec![
                    "\x1b[0m".to_string(),
                    left,
                    self.style.get_begin(),
                    bars.iter()
                        .zip(sizes)
                        .map(|(b, s)| b.render(s))
                        .collect::<Vec<String>>()
                        .join(""),
                    self.style.get_end(),
                    "\x1b[0m".to_string(),
                    right,
                    "\x1b[0m".to_string(),
                ]
                .join("")
            }
        }
    }
}
