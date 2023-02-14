use crate::compiler::read_token;
use crate::finder::start_find;

mod compiler;
mod hash;
mod finder;

use std::{time, path::Path};

fn main() {
    if !Path::new("files/hashed").exists() {
        compile();
    } else {
        println!("all files ready");
        let word = "abo";
        find_word(word.to_string());
    }
}

fn find_word(word: String) {
    start_find(word);
}

fn compile() {
    let start = time::Instant::now();
    read_token();

    println!("COMPILING COMPLETE\nTook: {} seconds", start.elapsed().as_secs());
}
