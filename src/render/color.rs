use colored::Color;

fn get_colorize_string(code: String) -> String {
    vec!["\x1b[".to_string(), code.clone(), "m".to_string()].join("")
}

pub enum Effect {
    Bold,
    Underline,
    Strikethrough,
    Italic,
}

impl Effect {
    fn to_effect_str() -> &'static str {}
}

pub struct TextStyle {
    bg_color: Option<Color>,
    fg_color: Option<Color>,
    effect: Option<Effect>,
}

impl TextStyle {
    fn apply(&self, input: String) {
        let reset = "\x1b[0m".to_string();
        let bg_begin = match self.bg_color {
            Some(Color) => get_colorize_string(self.bg_color.unwrap().to_bg_str().to_string()),
            Option::None => "".to_string(),
        };
        let fg_begin = match self.fg_color {
            Some(Color) => get_colorize_string(self.fg_color.unwrap().to_bg_str().to_string()),
            Option::None => "".to_string(),
        };
        let effect_begin = "".to_string();
    }
}
