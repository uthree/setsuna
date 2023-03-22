use crate::core::vector2::Vector2;
use crate::render::color::Effect;
use crate::render::color::{get_colorize_string, remove_color_settings};
use crate::render::renderable::RenderableLine;
use colored::Color;

#[derive(Default)]
pub struct TextStyle {
    bg_color: Option<Color>,
    fg_color: Option<Color>,
    effect: Option<Effect>,
}

impl TextStyle {
    pub fn apply(&self, input: String) -> String {
        let reset = "\x1b[0m".to_string();
        let bg_begin = match self.bg_color {
            Some(color) => get_colorize_string(self.bg_color.unwrap().to_bg_str().to_string()),
            Option::None => "".to_string(),
        };
        let fg_begin = match self.fg_color {
            Some(color) => get_colorize_string(self.fg_color.unwrap().to_bg_str().to_string()),
            Option::None => "".to_string(),
        };
        let effect_begin = match &self.effect {
            Some(effect) => {
                get_colorize_string(self.effect.clone().unwrap().to_effect_str().to_string())
            }
            Option::None => "".to_string(),
        };
        vec![
            reset.clone(),
            bg_begin,
            fg_begin,
            effect_begin,
            input,
            reset.clone(),
        ]
        .join("")
    }
}

pub struct Text {
    pub label: String,
    pub style: TextStyle,
}

impl RenderableLine for Text {
    fn render(&self) -> String {
        self.style.apply(self.label.clone())
    }
    fn size(&self) -> u16 {
        remove_color_settings(self.label.clone())
            .len()
            .try_into()
            .unwrap()
    }
}

impl Default for Text {
    fn default() -> Self {
        Text {
            label: "".to_string(),
            style: TextStyle::default(),
        }
    }
}

impl Text {
    pub fn new(s: &str) -> Self {
        let mut a = Text::default();
        a.label = s.to_string();
        a
    }
}
