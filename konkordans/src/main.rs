use crate::{compiler::read_token, finder::latin1_to_string};
use crate::finder::start_find;
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

mod compiler;
mod finder;
mod hash;

use std::{
    env::{args, Args},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time,
};

fn main() {
    if !Path::new("files/hashed").exists() {
        compile();
    } else {
        // test();
        // Debug purpose
        // let arg = "för";
        // ------------

        let arg = args().nth(1).expect("no word?");
        // // println!("all files ready");
        // // println!("arg is: {}", arg);

        find_word(arg.trim().to_lowercase().to_string());
    }
}

fn test() {
    let word: &str = "för";
    // 3362858, 3395293
    let line1: usize = 3362858;
    let line2: usize = 3395293;
    

    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open("files/token").unwrap()),
    );
    let mut lines = buf.lines().skip(line1 -1);

    let mut tmp = latin1_to_string(lines.next().unwrap().unwrap().as_bytes());

    let mut buf2 = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open("files/token").unwrap()),
    );
    let mut lines2 = buf2.lines().skip(line2 -1);

    let mut tmp2 = latin1_to_string(lines2.next().unwrap().unwrap().as_bytes());

    println!("tmp st: {:?}", tmp.split_whitespace().next().unwrap());
    println!("tmp is: {:?}", tmp.split_whitespace().next().unwrap().as_bytes());

    println!("tmp2 st: {:?}", tmp2.split_whitespace().next().unwrap());
    println!("tmp2 is: {:?}", tmp2.split_whitespace().next().unwrap().as_bytes());
}

fn find_word(word: String) {
    start_find(word);
}

fn compile() {
    println!("compiling...");
    let start = time::Instant::now();
    read_token();

    println!(
        "COMPILING COMPLETE\nTook: {} seconds",
        start.elapsed().as_secs()
    );
}
