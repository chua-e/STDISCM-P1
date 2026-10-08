use chrono::{DateTime, Utc};

pub type PrimeTime = Vec<(u64, DateTime<Utc>)>;

/**
 * Checks if a given number is a prime number
 */
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

/**
 * Checks if a number is divisible with all the numbers
 * in the provided range
 */
pub fn has_divisor (n: u64, start: u64, end: u64) -> bool {
    (start..=end).any(|d| n % d == 0)
}