mod general;
mod isolated;

use std::env;
use terminal_size::{Width, terminal_size};

use crate::{general::run_general, isolated::run_isolated};

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_genral = args.contains(&String::from("-g"));

    let terminal_width;

    if let Some((Width(width), _heigth)) = terminal_size() {
        terminal_width = width;
    } else {
        terminal_width = 50;
    }

    let width_usize = terminal_width as usize;
    println!("{text:^w$}", text = "SPACE", w = width_usize);

    if is_genral {
        run_general(terminal_width as f64 - 2.0);
    } else {
        run_isolated(terminal_width as f64 - 2.0);
    }
}
