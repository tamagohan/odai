use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn usage() {
    println!("rsgrep PATTERN FILENAME")
}

fn main() {
    let pattern = match env::args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };

    let filename = match env::args().nth(2) {
        Some(name) => name,
        None => {
            usage();
            return;
        }
    };

    let re = match Regex::new(&pattern) {
        Ok(re) => re,
        Err(e) => {
            println!("An error occured while compiling pattern {}:{}", pattern, e);
            return;
        }
    };

    let f = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("An error occured while opening file {}:{}", filename, e);
            return;
        }
    };
    let bf = BufReader::new(f);
    for line in bf.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("An error occured while reading a line {}", e);
                return;
            }
        };
        if re.is_match(&line) {
            println!("{}", line);
        }
    }
}
