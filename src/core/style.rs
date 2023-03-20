use crate::core::render::Render;
use crate::core::vector2::Vector2;
use colored::Color;

#[derive(Clone, Copy)]
pub enum TextEffect {
    Clear,
    Bold,
    Dimmed,
    Underline,
    Reversed,
    Italic,
    Blink,
    Hidden,
    Strikethrough,
}

#[derive(Clone)]
pub struct TextStyle {
    pub bg_color: Color,
    pub fg_color: Color,
    pub effects: Vec<TextEffect>,
}

#[derive(Clone, Default)]
pub struct Text {
    pub string: String,
    pub style: TextStyle,
}

#[derive(Clone, Default)]
pub struct OneLineBlockStyle {
    pub style: TextStyle,
    pub left_text: Text,
    pub right_text: Text,
}

#[derive(Clone)]
enum OneLineBlockContent {
    None,
    Text(Text),
}

#[derive(Clone, Default)]
pub struct OneLineBlock {
    style: OneLineBlockStyle,
    content: OneLineBlockContent,
}

pub struct MultiLineBlockStyle {}

impl Default for TextEffect {
    fn default() -> Self {
        TextEffect::Clear
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            bg_color: Color::Black,
            fg_color: Color::White,
            effects: vec![],
        }
    }
}

impl Default for OneLineBlockContent {
    fn default() -> Self {
        OneLineBlockContent::None
    }
}

impl TextEffect {
    fn to_effect_str(&self) -> &'static str {
        match self {
            TextEffect::Clear => "0",
            TextEffect::Bold => "1",
            TextEffect::Dimmed => "2",
            TextEffect::Italic => "3",
            TextEffect::Underline => "4",
            TextEffect::Blink => "5",
            TextEffect::Reversed => "7",
            TextEffect::Hidden => "8",
            TextEffect::Strikethrough => "9",
        }
    }
}

fn colorize(s: String) -> String {
    vec!["\x1b[".to_string(), s, "m".to_string()].join("")
}

impl Render for Text {
    fn render(&self) -> Vec<Vec<char>> {
        let text = self.string.clone();
        let effect_str = self
            .style
            .effects
            .iter()
            .map(|e| colorize(e.to_effect_str().to_string()))
            .collect::<Vec<String>>()
            .join("");
        let bg_str = colorize(self.style.bg_color.to_bg_str().to_string());
        let fg_str = colorize(self.style.fg_color.to_fg_str().to_string());
        let reset_fg = colorize(Color::White.to_fg_str().to_string());
        let reset_bg = colorize(Color::Black.to_bg_str().to_string());
        let reset_effect = colorize(TextEffect::Clear.to_effect_str().to_string());
        let out_str = vec![
            effect_str,
            bg_str,
            fg_str,
            text,
            reset_fg,
            reset_bg,
            reset_effect,
        ]
        .join("");
        vec![out_str.chars().collect()]
    }
    fn size(&self) -> Vector2<u16> {
        let width = self.string.len() as u16;
        let height = 1 as u16;
        return Vector2 {
            x: width,
            y: height,
        };
    }
}

impl Render for OneLineBlock {
    fn render(&self) -> Vec<Vec<char>> {
        match &self.content {
            OneLineBlockContent::None => vec![vec![]],
            OneLineBlockContent::Text(text) => {
                let text_chars = text.render()[0].clone();
                vec![text_chars]
            }
        }
    }
    fn size(&self) -> Vector2<u16> {
        match &self.content {
            OneLineBlockContent::None => Vector2 { x: 0, y: 0 },
            OneLineBlockContent::Text(text) => text.size(),
        }
    }
}
