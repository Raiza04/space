mod general;
mod isolated;

use terminal_size::{Width, terminal_size};

use crate::general::run_general;

fn main() {
    let terminal_width;

    if let Some((Width(width), _heigth)) = terminal_size() {
        terminal_width = width;
    } else {
        terminal_width = 50;
    }

    let width_usize = terminal_width as usize;
    println!("{text:^w$}", text = "SPACE", w = width_usize);

    run_general(terminal_width as f64);
}
