use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T> Add<Self> for Vec2<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2::new(rhs.x + self.x, self.y + rhs.y)
    }
}

impl<T> Sub<Self> for Vec2<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2::new(rhs.x - self.x, self.y - rhs.y)
    }
}

impl<T> Div<Self> for Vec2<T>
where
    T: Div<T, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec2::new(rhs.x / self.x, self.y / rhs.y)
    }
}

impl<T> Mul<Self> for Vec2<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec2::new(rhs.x * self.x, self.y * rhs.y)
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T>,
    T: Clone,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Vec2::new(self.x * rhs.clone(), self.y * rhs.clone())
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Clone,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Vec2::new(self.x / rhs.clone(), self.y / rhs.clone())
    }
}

impl<T> Add<T> for Vec2<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Vec2::new(self.x + rhs.clone(), self.y + rhs.clone())
    }
}

impl<T> Sub<T> for Vec2<T>
where
    T: Sub<Output = T> + Clone,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Vec2::new(self.x - rhs.clone(), self.y - rhs.clone())
    }
}

use std::fmt;
impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("({}, {})", self.x.to_string(), self.y.to_string())
        )
    }
}
