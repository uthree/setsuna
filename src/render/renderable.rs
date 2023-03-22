use crate::core::vector2::Vector2;
use ndarray::Array2;

pub trait Renderable {
    fn size(&self) -> Vector2;
    fn render(&self) -> Array2<char>;
}
