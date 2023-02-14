use crate::compiler::read_token;

mod compiler;

use std::time;

fn main() {
    println!("Hello, world!");
    let start = time::Instant::now();
    read_token();

    println!("COMPILING COMPLETE\nTook: {} seconds", start.elapsed().as_secs());
}
