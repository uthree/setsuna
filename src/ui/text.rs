use crate::core::vector2::Vector2;
use crate::ui::line::Line;
use colored::Color;
use regex::Regex;

#[derive(Clone)]
pub enum Pivot {
    Left,
    Right,
    Center,
}

impl Default for Pivot {
    fn default() -> Self {
        Pivot::Center
    }
}

#[derive(Clone, Debug)]
pub enum Effect {
    Bold,
    Italic,
    Underline,
}

impl Effect {
    fn to_effect_str(&self) -> &str {
        match &self {
            Effect::Bold => "1",
            Effect::Italic => "3",
            Effect::Underline => "4",
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct TextStyle {
    pub background_color: Option<Color>,
    pub foreground_color: Option<Color>,
    pub effect: Option<Effect>,
}

impl TextStyle {
    pub fn get_begin(&self) -> String {
        let mut begin_bg = "".to_string();
        let mut begin_fg = "".to_string();
        let mut begin_effect = "".to_string();
        if self.background_color.is_some() {
            begin_bg = get_special_str(self.background_color.unwrap().to_bg_str().to_string());
        }
        if self.foreground_color.is_some() {
            begin_fg = get_special_str(self.foreground_color.unwrap().to_fg_str().to_string());
        }
        if self.effect.is_some() {
            begin_effect =
                get_special_str(self.effect.as_ref().unwrap().to_effect_str().to_string());
        }
        vec![begin_fg, begin_bg, begin_effect].join("")
    }
    pub fn get_end(&self) -> String {
        let mut reset = "\x1b[0m".to_string();
        if self.effect.is_none()
            && self.foreground_color.is_none()
            && self.background_color.is_none()
        {
            reset = "".to_string();
        }
        reset
    }
    pub fn apply(&self, i: String) -> String {
        vec![self.get_begin(), i, self.get_end()].join("")
    }
}

fn get_n_char_str(c: char, n: usize) -> String {
    (0..n).map(|_| c).collect::<String>()
}

fn get_special_str(style: String) -> String {
    vec!["\x1b[".to_string(), style, "m".to_string()].join("")
}

#[derive(Clone)]
pub struct Text {
    pub label: String,
    pub style: TextStyle,
    pub pivot: Pivot,
    pub padding_char: char,
}

impl Text {
    pub fn label(label: &str) -> Self {
        Text {
            label: label.to_string(),
            style: TextStyle::default(),
            pivot: Pivot::default(),
            padding_char: ' ',
        }
    }

    pub fn left(mut self) -> Self {
        self.pivot = Pivot::Left;
        self
    }

    pub fn right(mut self) -> Self {
        self.pivot = Pivot::Right;
        self
    }

    pub fn center(mut self) -> Self {
        self.pivot = Pivot::Center;
        self
    }
}

impl Line for Text {
    fn render(&self, size: usize) -> String {
        let pad_lr = size as isize - self.label.len() as isize;
        let pad_l = pad_lr / 2;
        let pad_r = pad_lr - pad_l;
        let pad_l = if pad_lr > 0 {
            get_n_char_str(self.padding_char, pad_l as usize)
        } else {
            "".to_string()
        };
        let pad_r = if pad_lr > 0 {
            get_n_char_str(self.padding_char, pad_r as usize)
        } else {
            "".to_string()
        };
        let mut begin_bg = "".to_string();
        let mut begin_fg = "".to_string();
        let mut begin_effect = "".to_string();
        let reset = get_special_str("0".to_string());
        let mut label = self.label.clone();
        if label.len() > size {
            label = self.label[0..1].to_string();
        }
        if self.style.background_color.is_some() {
            begin_bg =
                get_special_str(self.style.background_color.unwrap().to_bg_str().to_string());
        }
        if self.style.foreground_color.is_some() {
            begin_fg =
                get_special_str(self.style.foreground_color.unwrap().to_fg_str().to_string());
        }
        if self.style.effect.is_some() {
            begin_effect = get_special_str(
                self.style
                    .effect
                    .as_ref()
                    .unwrap()
                    .to_effect_str()
                    .to_string(),
            );
        }
        let mut reset = vec![
            reset,
            begin_bg.clone(),
            begin_fg.clone(),
            begin_effect.clone(),
        ]
        .join("");
        if self.style.effect.is_none()
            && self.style.foreground_color.is_none()
            && self.style.background_color.is_none()
        {
            reset = "".to_string();
        }

        match self.pivot {
            Pivot::Left => vec![
                reset.clone(),
                begin_fg,
                begin_bg,
                label,
                pad_l,
                pad_r,
                reset.clone(),
            ]
            .join(""),
            Pivot::Right => vec![
                reset.clone(),
                begin_fg,
                begin_bg,
                pad_l,
                pad_r,
                label.clone(),
                reset.clone(),
            ]
            .join(""),
            Pivot::Center => vec![
                reset.clone(),
                begin_fg,
                begin_bg,
                pad_l,
                label.clone(),
                pad_r,
                reset.clone(),
            ]
            .join(""),
        }
    }
}
