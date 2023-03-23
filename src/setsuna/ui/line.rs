pub trait RenderLine {
    fn render(&self) -> String;
    fn len(&self) -> usize;
}

pub trait RenderLineResizable {
    fn render(&self, size: usize) -> String;
}

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
