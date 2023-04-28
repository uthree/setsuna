pub trait Line {
    fn render(&self, length: usize) -> String;
}
