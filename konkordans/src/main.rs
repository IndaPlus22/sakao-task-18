use crate::compiler::read_token;
use crate::finder::start_find;

mod compiler;
mod hash;
mod finder;

use std::{time, path::Path, env::{Args, args}};

fn main() {
    if !Path::new("files/hashed").exists() {
        compile();
    } else {
        let arg = args().nth(1).expect("no word?");
        // println!("all files ready");
        // println!("arg is: {}", arg);

        // let arg = "targetdebugkonkordans.exe";
        find_word(arg.to_lowercase().to_string());
    }
}

fn find_word(word: String) {
    start_find(word);
}

fn compile() {
    println!("compiling...");
    let start = time::Instant::now();
    read_token();

    println!("COMPILING COMPLETE\nTook: {} seconds", start.elapsed().as_secs());
}
