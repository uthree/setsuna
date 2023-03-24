use termion::event::{Event, Key};
pub trait Window {
    fn recieve_event(&mut self, key: Event);
}
