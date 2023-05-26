use crate::core::vector2::Vector2;

#[derive(Clone, Copy)]
pub struct Rectangle<T>
where
    T: Copy,
{
    pub top_left: Vector2<T>,
    pub bottom_right: Vector2<T>,
}
