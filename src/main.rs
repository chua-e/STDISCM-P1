use std::io::BufRead;
use std::{io};
use std::fs::File;
use std::path::Path;
use STDISCM_P1::thread::make_threads;

fn main() {
    let filename = "config.txt";
    let (n_threads, n_range) = read_file(filename);

    make_threads(n_threads, n_range);
}

fn read_file(filename: &str) -> (u64, Vec<u64>) {
    let mut x: u64 = 0;
    let mut y: Vec<u64> = Vec::new();

    let path = Path::new(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return (0, vec![]);
        }
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(text) => {
                // println!("{}\n", text);

                let mut tokens = text.split_whitespace();
                let first_token = tokens.next().unwrap_or("");
                let second_token = tokens.next().unwrap_or("");

                if first_token == "x" {
                    if let Ok(threads) = second_token.parse::<u64>() {
                        x = threads;
                    }
                } else if first_token == "y" {
                    let mut range_parts = second_token.split('-');
                    if let (Some(n1_str), Some(n2_str)) = (range_parts.next(), range_parts.next()) {
                        if let (Ok(n1), Ok(n2)) = (n1_str.parse::<u64>(), n2_str.parse::<u64>()) {
                            y.push(n1);
                            y.push(n2);
                        }
                    }
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("Number of threads: {}\nTotal number range: ({}, {})\n", x, y[0], y[1]);

    return (x, y);
}