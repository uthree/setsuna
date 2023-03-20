use crate::core::vector2::Vector2;

pub trait Render {
    fn render(&self) -> Vec<Vec<char>>;
    fn size(&self) -> Vector2<u16>;
}
