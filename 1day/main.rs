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
    // pair of 2
    for i in &v {
        let want = 2020 - i;
        if v.contains(&want) {
            println!("{}and {}", i, want);
        }
    }
    // tripples
    
    for i in &v {
        let parent = 2020 - i;
        for j in &v {
            if parent > *j {
                let child = parent - *j;
                if v.contains(&child) {
                    println!("{}and {} and {}", j, child, i);
                }
            }
        }
    }




    Ok(())
}
