use crate::core::vector2::Vector2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rectangle<T>
where
    T: Clone + Copy,
{
    pub top_left: Vector2<T>,
    pub bottom_right: Vector2<T>,
}

impl<T> Rectangle<T>
where
    T: Clone + Copy,
{
    pub fn new(top_left: Vector2<T>, bottom_right: Vector2<T>) -> Self {
        Rectangle {
            top_left,
            bottom_right,
        }
    }
}
