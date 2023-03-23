use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vector2<T>
where
    T: Copy + Clone,
{
    x: T,
    y: T,
}

impl<T> Vector2<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vector2<T>
where
    T: Copy,
{
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
