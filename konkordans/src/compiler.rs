use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::thread::current;
use std::{fmt, path::Path};

const KORPUS_FILE: &str = "files/korpus";
const TOKEN_FILE: &str = "files/token.txt";

pub fn read_token() {
    if !Path::new("files/index").exists() {
        construct_index_file();
    } else {
        println!("already there");
    }

    construct_magic_file();
}

fn construct_index_file() {
    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open(TOKEN_FILE).unwrap()),
    );
    // File::open(TOKEN_FILE).unwrap()
    // OpenOptions::new().read(true).open(TOKEN_FILE).unwrap()

    let mut line = String::new();
    let mut bytes: usize = buf.read_line(&mut line).unwrap();
    let mut last_key: String = String::new();
    // let mut current_byte: usize = 0;
    let mut index = String::new();

    while bytes > 0 {
        let key = line.split_whitespace().next().unwrap();
        let byte_offset = line.split_whitespace().last().unwrap();

        if last_key != key {
            index += "\n";
            index += key;
        }
        index += " ";
        index += byte_offset;

        last_key = key.to_string();
        line.clear();
        bytes = buf.read_line(&mut line).unwrap();
        // let key = line.split_whitespace().next().unwrap();
        // let hash = hash_three(key);

        // if tokens.len() <= hash as usize {
        //     tokens.resize(hash + 1, Vec::new());
        // }
        // if last_key != key {
        //     tokens[hash as usize].push((key.to_string(), current_byte as u64));
        // }

        // last_key = key.to_string();
        // current_byte += bytes;

        // line.clear();
        // bytes = buf.read_line(&mut line).unwrap();
    }

    let mut file = File::create("files/index").expect("coudnt make index file");
    file.write_all(index.as_bytes());

    println!("yey");
}

fn construct_magic_file() -> Vec<Vec<(String, u64)>> {
    let mut tokens: Vec<Vec<(String, u64)>> = Vec::with_capacity(1000);
    let mut buf = BufReader::new(File::open("files/index").unwrap());

    let mut line = String::new();
    let mut bytes: usize = buf.read_line(&mut line).unwrap();
    let mut last_key: String = String::new();
    let mut current_byte: usize = 0;

    bytes = buf.read_line(&mut line).unwrap();
    while bytes > 0 {
        let key = line.split_whitespace().next().unwrap();
        let hash = hash_three(key);

        if tokens.len() <= hash as usize {
            tokens.resize(hash + 1, Vec::new());
        }
        if last_key != key {
            tokens[hash as usize].push((key.to_string(), current_byte as u64));
        }

        last_key = key.to_string();
        current_byte += bytes;

        line.clear();
        bytes = buf.read_line(&mut line).unwrap();
    }

    tokens
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
