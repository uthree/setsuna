use crate::setsuna::core::vector2::Vector2;

pub trait RenderBlockResizable {
    fn render(&self, size: Vector2<usize>) -> Vec<String>;
}
