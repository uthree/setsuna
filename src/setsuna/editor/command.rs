use crate::setsuna::editor::text_editor::Mode;
use crate::setsuna::editor::text_editor::TextEditor;
use std::ffi::OsString;

pub trait Execute {
    fn execute(&self, editor: &mut TextEditor);
}

pub struct ChangeMode {
    mode: Mode,
}

impl ChangeMode {
    pub fn new(mode: Mode) -> Self {
        ChangeMode { mode }
    }
}

impl Execute for ChangeMode {
    fn execute(&self, editor: &mut TextEditor) {
        editor.mode = self.mode.clone();
    }
}
