use primes::{Sieve, PrimeSet};

pub fn find_prime(start: &i32, end: &i32) {
    let mut pset = Sieve::new();
    let mut prime_set: Vec<i32> = Vec::new();

    let start_val = (*start).max(0) as u64;
    let end_val = (*end).max(0) as u64;

    pset.iter().take_while(|&n| n <= end_val).filter(|&n| n >= start_val).enumerate().for_each(|(ix, n)| {
        println!("Prime {}: {}", ix+1, n);
    });
}