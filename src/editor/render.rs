use crate::geometry::Vec2;
use crate::terminal::character::Character;
use ndarray::prelude::*;

pub trait Render {
    fn render(&self, size: Vec2<usize>) -> Array2<Character>;
}
