use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let mut v: Vec<u32> = Vec::new();

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        match line?.trim().parse() {
            Ok(num) => v.push(num),
            Err(err) => {
                println!("invalid line {}", err);
                continue;
            }
        };
    }

    // filter all > 2020
    v.retain(|&i| i < 2020);

    for i in v {

    }

    Ok(())
}
