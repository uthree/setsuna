use crate::setsuna::editor::{command::Command, text_editor::Mode};
use std::collections::HashMap;
use termion::event::{Event, Key};

pub struct KeyBinds {
    pub normal: HashMap<Event, Command>,
    pub insert: HashMap<Event, Command>,
}

impl Default for KeyBinds {
    fn default() -> Self {
        let mut normal = HashMap::<Event, Command>::new();
        let mut insert = HashMap::<Event, Command>::new();
        normal.insert(Event::Key(Key::Esc), Command::ChangeMode(Mode::Insert));
        KeyBinds { normal, insert }
    }
}
