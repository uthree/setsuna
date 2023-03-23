use crate::setsuna::ui::line::{Pivot, RenderLineResizable};
use crate::setsuna::ui::text::{Text, TextStyle};
use regex::Regex;

pub enum BarContent {
    Text(Text),
    Bars(Vec<Bar>),
}

pub struct Bar {
    pub content: BarContent,
    pub style: TextStyle,
    pub left: String,
    pub right: String,
}

fn remove_special(i: &String) -> String {
    let re = Regex::new(r"\x1b\x5b(.*)m").unwrap();
    re.replace_all(i, "").into()
}

impl RenderLineResizable for Bar {
    fn render(&self, size: usize) -> String {
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
                let len_l = remove_special(&self.left).len();
                let len_r = remove_special(&self.right).len();

                vec![
                    left,
                    self.style.get_begin(),
                    text.clone().render(size - len_l - len_r),
                    self.style.get_end(),
                    right,
                ]
                .join("")
            }
            BarContent::Bars(bars) => {
                let len_l = remove_special(&self.left).len();
                let len_r = remove_special(&self.right).len();
                let s = size - len_l - len_r;
                let sizes = (0..bars.len())
                    .map(|i| (s + i) / bars.len())
                    .collect::<Vec<usize>>();
                vec![
                    left,
                    self.style.get_begin(),
                    bars.iter()
                        .zip(sizes)
                        .map(|(b, s)| b.render(s))
                        .collect::<Vec<String>>()
                        .join(""),
                    self.style.get_end(),
                    right,
                ]
                .join("")
            }
        }
    }
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
}
