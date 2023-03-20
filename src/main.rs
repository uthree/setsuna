mod core;

use crate::core::render::Render;
use crate::core::style::Text;
fn main() {
    let mut t = Text::default();
    t.string = "Hello World!".to_string();
    println!("{}", t.render()[0].clone().iter().collect::<String>())
}
