use crate::setsuna::editor::{
    command::{ChangeMode, Execute},
    text_editor::Mode,
};
use std::collections::HashMap;
use termion::event::{Event, Key};

pub struct KeyBinds {
    pub normal: HashMap<Event, Box<dyn Execute>>,
    pub insert: HashMap<Event, Box<dyn Execute>>,
}

impl Default for KeyBinds {
    fn default() -> Self {
        let mut normal = HashMap::<Event, Box<dyn Execute>>::new();
        let mut insert = HashMap::<Event, Box<dyn Execute>>::new();
        insert.insert(
            Event::Key(Key::Esc),
            Box::new(ChangeMode::new(Mode::Normal)),
        );
        KeyBinds { normal, insert }
    }
}
