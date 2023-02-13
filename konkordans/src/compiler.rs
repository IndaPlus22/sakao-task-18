use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::thread::current;

const KORPUS_FILE: &str = "files/korpus";
const TOKEN_FILE: &str = "files/token.txt";

pub fn read_token() -> Vec<Vec<(String, u64)>> {
    let mut tokens: Vec<Vec<(String, u64)>> = Vec::with_capacity(1000);

    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open(TOKEN_FILE).unwrap()),
    );
    // File::open(TOKEN_FILE).unwrap()
    // OpenOptions::new().read(true).open(TOKEN_FILE).unwrap()

    let mut line = String::new();
    let mut bytes: usize = buf.read_line(&mut line).unwrap();
    let mut last_key = String::new();
    let mut current_byte: usize = 0;

    while bytes > 0 {
        let key = line.split_whitespace().next().unwrap();
        let hash = hash_three(key);

        if tokens.len() <= hash as usize {
            tokens.resize(hash+1, Vec::new());
        }
        if last_key != key {
            tokens[hash as usize].push((key.to_string(), current_byte as u64));
        }

        last_key = key.to_string();
        current_byte += bytes;

        line.clear();
        bytes = buf.read_line(&mut line).unwrap();
    }
    println!("yey");
    tokens
}

fn construct_magic_file(tokens: Vec<Vec<(String, u64)>>) {

}

fn hash_three(word: &str) -> usize {
    let chars = word.chars();

    let mut hash: usize = 0;
    let mut n = 0;
    for c in chars {
        if n == 3 {
            break;
        }
        hash = hash.wrapping_mul(17).wrapping_add(c as usize);
        n += 1;
    }

    hash
}
