#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    TrueColor { r: u8, g: u8, b: u8 },
}

impl Default for Color {
    fn default() -> Color {
        Color::White
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Effect {
    bold: bool,
    italic: bool,
    underline: bool,
    strike_through: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Style {
    pub effect: Effect,
    pub bg_color: Color,
    pub fg_color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub character: char,
    pub style: Style,
}

impl From<char> for Character {
    fn from(c: char) -> Character {
        Character {
            character: c,
            style: Style::default(),
        }
    }
}

impl Style {
    pub fn colorize_command(&self) -> String {
        let fg = match self.fg_color {
            Color::None => "".to_string(),
            Color::Black => "\x1b\x5b30m".to_string(),
            Color::Red => "\x1b\x5b31m".to_string(),
            Color::Green => "\x1b\x5b32m".to_string(),
            Color::Yellow => "\x1b\x5b33m".to_string(),
            Color::Blue => "\x1b\x5b34m".to_string(),
            Color::Magenta => "\x1b\x5b35m".to_string(),
            Color::Cyan => "\x1b\x5b36m".to_string(),
            Color::White => "\x1b\x5b37m".to_string(),
            Color::TrueColor { r, g, b } => format!("\x1b\x5b38;2;{};{};{}m", r, g, b),
        };

        let bg = match self.fg_color {
            Color::None => "".to_string(),
            Color::Black => "\x1b\x5b40m".to_string(),
            Color::Red => "\x1b\x5b41m".to_string(),
            Color::Green => "\x1b\x5b42m".to_string(),
            Color::Yellow => "\x1b\x5b43m".to_string(),
            Color::Blue => "\x1b\x5b44m".to_string(),
            Color::Magenta => "\x1b\x5b45m".to_string(),
            Color::Cyan => "\x1b\x5b46m".to_string(),
            Color::White => "\x1b\x5b47m".to_string(),
            Color::TrueColor { r, g, b } => format!("\x1b\x5b48;2;{};{};{}m", r, g, b),
        };

        let mut style = String::new();
        if self.effect.bold {
            style += "\x1b\x5b1m"
        }
        if self.effect.italic {
            style += "\x1b\x5b3m"
        }
        if self.effect.underline {
            style += "\x1b\x5b4m"
        }
        if self.effect.strike_through {
            style += "\x1b\x5b9m"
        }

        format!("\x1b\x5b0m{}{}{}", style, bg, fg)
    }
}
