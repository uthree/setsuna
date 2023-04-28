use crate::core::vector2::Vector2;
use crate::ui::block::Block;
use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Default, Debug, Clone)]
struct EditorTab {
    pub title: String,
    buffer: ropey::Rope,
}

impl EditorTab {
    fn load_file(&mut self, path: String) -> Result<(), std::io::Error> {
        self.buffer = Rope::from_reader(BufReader::new(File::open(path.clone())?))?;
        self.title = path;
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
struct TextEditor {
    active_tab_id: usize,
    tabs: Vec<EditorTab>,
}

impl Block for TextEditor {
    fn render(&self, size: Vector2<usize>) -> Vec<String> {
        vec![]
    }
}
