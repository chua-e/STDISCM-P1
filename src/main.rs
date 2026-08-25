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
                // println!("{}\n", text);

                let mut tokens = text.split_whitespace();
                let first_token = tokens.next().unwrap_or("");
                let second_token = tokens.next().unwrap_or("");

                if first_token == "x" {
                    if let Ok(threads) = second_token.parse::<i32>() {
                        n_threads = threads;
                    }
                } else if first_token == "y" {
                    let mut range_parts = second_token.split('-');
                    if let (Some(n1_str), Some(n2_str)) = (range_parts.next(), range_parts.next()) {
                        if let (Ok(n1), Ok(n2)) = (n1_str.parse::<i32>(), n2_str.parse::<i32>()) {
                            n_range.push(n1);
                            n_range.push(n2);
                        }
                    }
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    println!("number of threads: {}\nnumber range: ({}, {})", n_threads, n_range[0], n_range[1]);
}