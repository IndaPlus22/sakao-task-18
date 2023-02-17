use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::thread::current;
use std::{fmt, path::Path};

use crate::hash::hash_three;
// use crate::finder::latin1_to_string;

const KORPUS_FILE: &str = "files/korpus";
const TOKEN_FILE: &str = "files/token";

pub fn read_token() {
    if !Path::new("files/index").exists() {
        construct_index_file();
    } else {
        println!("already there");
    }

    construct_hashed_file();
}

fn construct_index_file() {
    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(File::open(TOKEN_FILE).unwrap()),
    );

    // let mut buf = BufReader::new(File::open(TOKEN_FILE).unwrap());
    // let token_content: String = fs::read_to_string("files/token")
    //     .unwrap()
    //     .parse()
    //     .unwrap();
    // let lines: Vec<&str> = token_content.split('\n').collect();

    let mut line = String::new();
    let mut bytes: usize = buf.read_line(&mut line).unwrap();
    let mut last_key: String = String::new();
    let mut index = String::new();

    // for i in 1..lines.len() - 1 {
    //     let key = lines[i].split_whitespace().next().unwrap();
    //     let byte_offset = lines[i].split_whitespace().last().unwrap();

    //     // if it is a new key do a newline and put in what key it is.
    //     if last_key != key {
    //         index += "\n";
    //         index += key;
    //     }
    //     index += " ";
    //     index += byte_offset;

    //     last_key = key.to_string();
    // }
    
    bytes = buf.read_line(&mut line).unwrap();
    while bytes > 0 {
        let key = line.split_whitespace().next().unwrap();
        let byte_offset = line.split_whitespace().last().unwrap();

        // if it is a new key do a newline and put in what key it is.
        if last_key != key {
            index += "\n"; // There is one problem with this and it is that the file will start with a new line but it is countered at ln:69
            index += key;
        }
        index += " ";
        index += byte_offset;

        last_key = key.to_string();
        line.clear();
        bytes = buf.read_line(&mut line).unwrap();
    }

    let mut file = File::create("files/index").expect("coudnt make index file");
    file.write_all(index.as_bytes());

    println!("yey");
}

fn construct_hashed_file() -> Vec<Vec<(String, u64)>> {
    let mut tokens: Vec<Vec<(String, u64)>> = Vec::with_capacity(1000);
    let mut buf = BufReader::new(File::open("files/index").unwrap());

    let mut line = String::new();
    let mut bytes: usize = buf.read_line(&mut line).unwrap();
    let mut last_key: String = String::new();
    let mut current_byte: usize = 0;

    // reading again because the first line empty becuase my code does it.
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

    let mut tmp = String::new();
    for i in 0..tokens.len() {
        if !tokens[i].is_empty() {
            // to know how many elements there are
            // tmp += tokens[i].len().to_string().as_str();
            // tmp += " ";
            for j in 0..tokens[i].len() {
                tmp += &tokens[i][j].0;
                tmp += " ";
                tmp += &tokens[i][j].1.to_string();

                tmp += " ";
            }
            tmp += "\n";
        } else {
            tmp += "\n";
        }
    }

    let mut file = File::create("files/hashed").expect("coudnt make hashed file");
    file.write_all(tmp.as_bytes());

    tokens
}

// Got help from jblomlof's solution
// got to know that å,ä,ö are the same in token. In my case I change it to unicode chars
fn fix_word(word: &str) -> String {
    let mut fixed = String::new();

    for c in word.bytes() {
        if c == 189 {
            // println!("c: {}", c);
            fixed.pop();
            fixed.pop();
            fixed.push(228 as char); // adding 'ä'
        } else {
            fixed.push(c as char);
        }
    }

    fixed
}
