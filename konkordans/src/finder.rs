use crate::hash::hash_three;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::thread::current;
use std::{fmt, path::Path};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub fn start_find(word: String) {
    let key = hash_three(&word);

    let file = File::open("files/hashed").expect("cant find hashed file when start find");
    let reader = BufReader::new(file);

    // skip to the hashed key line
    let mut lines = reader.lines().skip(key);

    // Read the key line
    let line = lines.next().unwrap().unwrap();
    // println!("ln: {}: \n{}", key +1, line);
    let mut tmp: Vec<&str> = line.trim().split_whitespace().map(|x| x).collect();

    let mut table: Vec<(&str, usize)> = Vec::new();

    for i in (0..tmp.len() - 1).step_by(2) {
        table.push((tmp[i], tmp[i + 1].parse().unwrap()));
    }

    // println!("table: {:?}", table);
    search(word, table);
}

fn search(word: String, table: Vec<(&str, usize)>) {
    // let w_prefix = &word[..3];
    let mut min = 0;
    let mut max = table.len();
    let key = hash_three(&word);

    let mut index_byte: usize = 0;
    while true {
        let m = (min + max) / 2;
        let other_key = hash_three(&table[m].0);

        if table[m].0 == word {
            index_byte = table[m].1;
            break;
        } else if is_larger(&table[m].0, &word.as_str()) == 1 {
            min = m;
        } else {
            max = m;
        }
    }
    println!("found index byte: {}", index_byte);

    let mut index_f = File::open("files/index").expect("cant find hashed file when start find");
    // skip to the hashed key line
    index_f.seek(SeekFrom::Start(index_byte as u64 + 1)).unwrap();
    let mut tmp = String::new();
    let rdr = BufReader::new(index_f);
    let line: String = rdr.lines().next().unwrap().unwrap();
    
    // println!("line in index: {}", line);
    print_res(line);
}

// returns 0 if min is larger, 1 if max is larger, 2 if equal
fn is_larger(min: &str, max: &str) -> u8 {
    if min == max {
        return 2;
    }

    if min.len() > max.len() {
        return 0;
    }
    if min.len() < max.len() {
        return 1;
    }
    for i in 0..max.len() {
        if min.chars().next().unwrap() < max.chars().next().unwrap() {
            return 1;
        } else if min.chars().next().unwrap() > max.chars().next().unwrap() {
            return 0;
        }
    }
    2
}

fn print_res(line: String) {
    let mut bytes: Vec<&str> = line.split_whitespace().collect();
    bytes.remove(0);

    println!("Det finns {} förekomster av ordet.", bytes.len());

    for byte in bytes {
        println!("{}", get_from_korpus(byte.parse().unwrap()));
    }
}

fn get_from_korpus(byte_offset: usize) -> String {
    let start_byte = byte_offset - 30;
    let end_byte = byte_offset + 35;

    let mut korpus_f = File::open("files/korpus").expect("cant find korpus");
    // skip to the hashed key line
    korpus_f.seek(SeekFrom::Start(start_byte as u64 + 1)).unwrap();

    let mut output = String::new();
    let mut buf = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(korpus_f),
    );

    buf.take((end_byte - start_byte) as u64).read_to_string(&mut output).unwrap();
    output.replace('\n', " ")
}
