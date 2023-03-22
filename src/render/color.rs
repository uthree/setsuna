use crate::render::renderable::RenderableLine;
use colored::Color;
use regex::Regex;

pub fn get_colorize_string(code: String) -> String {
    vec!["\x1b[".to_string(), code.clone(), "m".to_string()].join("")
}

pub fn remove_color_settings(input: String) -> String {
    let r = Regex::new("\x1b\\\x5b(.*)m").unwrap();
    r.replace_all(&input, "").to_string()
}

#[derive(PartialEq, Clone, Copy)]
pub enum Effect {
    Bold,
    Underline,
    Strikethrough,
    Italic,
}

impl Effect {
    pub fn to_effect_str(&self) -> &'static str {
        match self {
            Effect::Bold => "1",
            Effect::Underline => "4",
            Effect::Strikethrough => "9",
            Effect::Italic => "9",
        }
    }
}
