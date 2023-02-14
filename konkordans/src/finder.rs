use crate::hash::hash_three;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::thread::current;
use std::{fmt, path::Path};

use encoding_rs::mem::{
    convert_latin1_to_str, convert_latin1_to_str_partial, convert_latin1_to_utf8,
};
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
    loop {
        let m = (min + max) / 2;

        if table[m].0 == word {
            index_byte = table[m].1;
            break;
        } else if is_larger(&table[m].0, &word.as_str()) == 1 {
            min = m;
        } else if is_larger(&table[m].0, &word.as_str()) == 0 {
            max = m;
        }

        if max - min < 2 {
            println!("ordet fanns inte");
            return;
        }
    }
    // println!("found index byte: {}", index_byte);

    let mut index_f = File::open("files/index").expect("cant find hashed file when start find");
    // skip to the hashed key line
    index_f
        .seek(SeekFrom::Start(index_byte as u64 + 1))
        .unwrap();
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
    let min_v: Vec<u8> = min.chars().map(|x| x as u8).collect::<Vec<u8>>();
    let max_v: Vec<u8> = max.chars().map(|x| x as u8).collect::<Vec<u8>>();

    for i in 0..std::cmp::min(max.len(), min.len()) {
        if min_v[i] < max_v[i] {
            return 1;
        } else if min_v[i] > max_v[i] {
            return 0;
        }
    }
    if min.len() < max.len() {
        return 1;
    } else if min.len() > max.len() {
        return 0;
    }

    2
}

fn print_res(line: String) {
    let mut bytes: Vec<&str> = line.split_whitespace().collect();
    let word_length = bytes[0].len();
    bytes.remove(0);

    println!("Det finns {} förekomster av ordet.", bytes.len());
    let mut show: bool = true;
    if bytes.len() > 25 {
        println!("Ordet förekommer mer än 25 gånger. Vill du ha förekomsterna utskrivna? \n\nSkriv Y för att visa");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if input.trim().to_lowercase() == "y" {
                show = true;
            } else {
                show = false;
            }
        } else {
            println!("okej, om du inte skriver något får du inget.");
            show = false;
        }
    }

    if show {
        for byte in bytes {
            println!("{}", get_from_korpus(byte.parse().unwrap(), word_length));
        }
    }
}

fn get_from_korpus(byte_offset: usize, word_len: usize) -> String {
    let start_byte = byte_offset - 30;
    let end_byte = byte_offset + word_len + 30;

    let mut korpus_f = File::open("files/korpus").expect("cant find korpus");
    // skip to the hashed key line
    korpus_f
        .seek(SeekFrom::Start(start_byte as u64 + 1))
        .unwrap();

    let mut output = String::new();
    let mut out: Vec<u8> = vec![0u8; 0];

    korpus_f
        .take((end_byte - start_byte) as u64)
        .read_to_end(&mut out)
        .unwrap();

    output = latin1_to_string(&out);
    output.replace('\n', " ")
}

fn latin1_to_string(s: &[u8]) -> String {
    s.iter().map(|&c| c as char).collect()
}
