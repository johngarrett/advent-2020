use std::ops::Range;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() {
    let path = "input";
    let mut entries: Vec<str> = Vec::new();

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        entries.push(line.unwrap().trim())
    }

    println!("Hello, world!");
}

fn is_valid(password: String, keyword: String, range: Range<u16>) -> bool {
    return false;
}
