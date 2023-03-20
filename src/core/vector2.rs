use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2<T>
where
    T: Clone + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> Add for Vector2<T>
where
    T: Clone + Add<Output = T> + Copy,
{
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vector2<T>
where
    T: Clone + Sub<Output = T> + Copy,
{
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Clone + Mul<Output = T> + Copy,
{
    type Output = Vector2<T>;

    fn mul(self, scalar: T) -> Vector2<T> {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Clone + Div<Output = T> + Copy,
{
    type Output = Vector2<T>;

    fn div(self, scalar: T) -> Vector2<T> {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}
