mod core;
mod render;
mod ui;

use crate::render::renderable::RenderableLine;
use crate::ui::bar::Bar;

fn main() {
    println!("{}", Bar::label("Hello").render());
}
