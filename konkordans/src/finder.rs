
use crate::hash::hash_three;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::thread::current;
use std::{fmt, path::Path};

pub fn start_find(word: String) {
    let key = hash_three(&word);

    let file = File::open("files/hashed").expect("cant find hashed file when start find");
    let reader = BufReader::new(file);

    // skip to the hashed key line
    let mut lines = reader.lines().skip(key);
    
    // Read the key line
    let line = lines.next().unwrap().unwrap();
    // println!("ln: {}: \n{}", key +1, line);
    let table: Vec<(String, usize)> = line
    .trim()
    .split_whitespace()
    .map(|(x, y)|);
}