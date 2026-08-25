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