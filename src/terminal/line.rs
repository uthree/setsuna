use crate::terminal::character::{Character, Color, Style};

pub struct Line {
    data: Vec<Character>,
}

impl Line {
    fn render(&self) -> String {
        String::new()
    }
}
