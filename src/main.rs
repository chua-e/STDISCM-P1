use std::io::BufRead;
use std::{io, println};
use std::io::{Write};
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use STDISCM_P1::threads::{variant_1, variant_2, variant_3, variant_4};

fn main() {
    let filename = "config.txt";
    let (n_threads, n_range) = read_file(filename).unwrap();

    println!("Choose the corresponding number for each variant:");
    println!("1 - Print immediately + Straight division (A1 B1)");
    println!("2 - Print immediately + Linear search (A2 B1)");
    println!("3 - Wait until all threads finish before print + Straight division (A1 B2)");
    println!("4 - Wait until all threads finish before print + Linear search (A2 B2)");
    println!("5 - Time comparison of all variants");
    println!("Type 'exit' to quit");

    loop {
        println!();
        print!("Input: ");
        io::stdout().flush().unwrap();

        let mut input = String::new(); 
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        println!();

        match input {
            "1" => {
                let start = Instant::now();
                variant_1(n_threads, n_range.clone(), true);

                let duration = start.elapsed();

                println!();
                println!("Time elapsed: {:?}", duration);
            },

            "2" => {
                let start = Instant::now();
                variant_2(n_threads, n_range.clone(), true);

                let duration = start.elapsed();

                println!();
                println!("Time elapsed: {:?}", duration);
            },

            "3" => {
                let start = Instant::now();
                variant_3(n_threads, n_range.clone(), true);

                let duration = start.elapsed();

                println!();
                println!("Time elapsed: {:?}", duration);
            },
            
            "4" => {
                let start = Instant::now();
                variant_4(n_threads, n_range.clone(), true);

                let duration = start.elapsed();

                println!();
                println!("Time elapsed: {:?}", duration);
            },

            "5" => {
                let mut times = Vec::new();

                let start_one = Instant::now();
                variant_1(n_threads, n_range.clone(), false);
                let duration_one = start_one.elapsed();
                times.push(duration_one);

                let start_two = Instant::now();
                variant_2(n_threads, n_range.clone(), false);
                let duration_two = start_two.elapsed();
                times.push(duration_two);

                let start_three = Instant::now();
                variant_3(n_threads, n_range.clone(), false);
                let duration_three = start_three.elapsed();
                times.push(duration_three);

                let start_four = Instant::now();
                variant_4(n_threads, n_range.clone(), false);
                let duration_four = start_four.elapsed();
                times.push(duration_four);
                

                for t in times {
                    println!("Time elapsed: {:?}", t);
                }
            },
            
            "exit" => break,
            _ => println!("Variant does not exist")
        }
    }
}

/**
 * Reads the config file in the format where x is number of threads and
 * y is the range of numbers for prime search
 */
fn read_file(filename: &str) -> Result<(u64, Vec<u64>), &'static str> {
    let mut x: u64 = 0;
    let mut y: Vec<u64> = Vec::new();

    let path = Path::new(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Ok((0, vec![]));
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
                            if n1 > n2 {
                                return Err("The starting number should be lesser than the end number.");
                            }
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

    return Ok((x, y));
}