use crate::setsuna::{core::vector2::Vector2, editor::text_editor::TextEditor};
use termion::event::{Event, Key};
pub trait Window {
    fn recieve_event(&mut self, key: Event);
}
