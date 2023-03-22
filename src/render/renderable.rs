use crate::core::vector2::Vector2;

pub trait Renderable {
    fn size(&self) -> Vector2<u16>;
    fn render(&self) -> Vec<String>;
}

pub trait RenderableLine {
    fn size(&self) -> u16;
    fn render(&self) -> String;
}
