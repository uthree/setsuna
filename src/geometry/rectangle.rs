use crate::geometry::vec2::Vec2;

pub struct Rectangle<T> {
    top_left: Vec2<T>,
    bottom_right: Vec2<T>,
}
