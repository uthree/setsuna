use crate::core::rectangle::Rectangle;
use crate::core::tab::Tab;
use crate::core::vector2::Vector2;

pub trait WindowBase {
    fn top_left(&self) -> Vector2<u16> {
        self.rect().top_left
    }
    fn bottom_right(&self) -> Vector2<u16> {
        self.rect().bottom_right
    }
    fn rect(&self) -> Rectangle<u16> {
        return Rectangle {
            top_left: self.top_left(),
            bottom_right: self.bottom_right(),
        };
    }
    fn size(&self) -> Vector2<u16> {
        self.bottom_right() - self.top_left()
    }
    fn render(&self) -> Vec<Vec<char>>;
    fn resize_rect(&mut self, rect: Rectangle<u16>) {
        let size = rect.bottom_right - rect.top_left;
        self.resize(size);
    }
    fn resize(&mut self, size: Vector2<u16>) {
        self.resize_rect(Rectangle::new(self.top_left(), size - self.top_left()));
    }
    fn is_active(&self) -> bool;
}

pub struct VerticalWindowSplit<'a> {
    left: &'a dyn WindowBase,
    right: &'a dyn WindowBase,
}

pub struct EditorWindow {
    tabs: Vec<Tab>,
}
