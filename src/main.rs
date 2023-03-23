pub mod setsuna;

use crate::setsuna::ui::bar::*;
use crate::setsuna::ui::line::RenderLineResizable;
use colored::Color;

fn main() {
    let mut b1 = Bar::label("* something1.rs");
    let mut b2 = Bar::label("something2.rs");
    let mut b3 = Bar::label("something3.rs");
    let mut bs = Bar::from_bars(vec![b1, b2, b3]);

    println!("{}", bs.render(40));
}
