use std::io::BufRead;
use std::{io};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("config.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    let mut n_threads = 0;
    let mut n_range: Vec<i32> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(text) => {
                println!("{}", text)
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}