use crate::setsuna::editor::text_editor::Mode;
use crate::setsuna::editor::text_editor::TextEditor;
use std::ffi::OsString;

pub enum Command {
    ChangeMode(Mode),
}

impl Command {
    pub fn execute(self, editor: &mut TextEditor) {
        match self {
            Command::ChangeMode(mode) => {
                editor.mode = mode;
            }
        }
    }
}
