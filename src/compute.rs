use primes::{Sieve, PrimeSet};
use std::time::{SystemTime};
use chrono::{DateTime, Utc};

pub type PrimeTime = Vec<(u64, DateTime<Utc>)>;

pub fn find_prime(start: &u64, end: &u64) -> PrimeTime {
    let mut pset = Sieve::new();

    pset.iter()
        .take_while(|&n| n <= *end)
        .filter(|&n| n >= *start)
        .map(|n| {
            let datetime: DateTime<Utc> = SystemTime::now().into();
            (n, datetime)
        })
        .collect()
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; } 
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let mut i = 5;
    while i*i <= n {
        if (n % i == 0) || (n % (i+2) == 0) {
            return false;
        }
        i += 6;
    }
    true
}