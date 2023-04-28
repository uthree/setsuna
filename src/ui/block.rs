use crate::core::vector2::Vector2;

pub trait Block {
    fn render(&self, size: Vector2<usize>) -> Vec<String>;
}
