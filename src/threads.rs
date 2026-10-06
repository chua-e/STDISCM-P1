use std::thread;
use std::sync::mpsc;
use crate::compute::{PrimeTime, find_prime, is_prime};
use chrono::{DateTime, Utc};

// print immediately, straight division
/**
 * Each thread gets a range to handle prime finding
 */
pub fn variant_1(n_thread: u64, n_range: Vec<u64>) {
    let range_list = allocate_ranges(n_thread, n_range);
    let (tx, rx) = mpsc::channel::<(usize, u64, DateTime<Utc>)>();

    for (i, (start, end)) in range_list.into_iter().enumerate() {
        let thread_id = i + 1;
        let tx = tx.clone();

        thread::spawn(move || {
            for n in start..=end {
                if is_prime(n) {
                    thread::sleep(std::time::Duration::from_millis(10));
                    tx.send((thread_id, n, Utc::now())).unwrap();
                }
            }
        });
    }
    
    drop(tx);

    for (id, prime, timestamp) in rx {
        println!("Thread {}: Prime {}, Timestamp: {:?}", id, prime, timestamp);
    }

    //fix
}

// print immediately, linear search
pub fn variant_2(n_thread: u64, n_range: Vec<u64>) {
    // let mut thread_list = Vec::new();

    for i in 1.. n_thread {

    }
    
    for i in n_range {

    }
}

// wait to print, straight division
pub fn variant_3 (n_thread: u64, n_range: Vec<u64>) {
    let range_list = allocate_ranges(n_thread, n_range);
    let mut thread_list = Vec::new();

    for (i, (start, end)) in range_list.into_iter().enumerate() {
        let thread_id = i + 1;
        let t = thread::spawn(move || {
           let primes = find_prime(&start, &end);
            (thread_id, start, end, primes)
        });

        thread_list.push(t);
    }
    
    for t in thread_list {
        if let Ok((id, start, end, primes)) = t.join() {
            println!("Thread {}", id);
            println!("Number range: ({}, {})", start, end);
            
            for (ix, (prime, _)) in primes.iter().enumerate() {
                println!("Prime {}: {}", ix + 1, prime);
            }
            println!();
        }
    }
}

// wait to print, linear search
pub fn variant_4() {

}


/**
 * Helper function that allocates numbers from the config range
 * into equal segments per thread, only returns the first and
 * last number of the segmented range
 */
fn allocate_ranges (n_thread: u64, n_range: Vec<u64>) -> Vec<(u64, u64)> {
    if n_range.len() < 2 || n_thread == 0 {
        return Vec::new();
    }

    let range_start = n_range[0];
    let range_end = n_range[1];
    let increment = (range_end - range_start)/n_thread;

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let mut current = range_start;
    for i in 0..n_thread {
        let temp_start = current;

        let temp_end = if i == n_thread-1 {
            range_end
        } else {
            current + increment
        };

        ranges.push((temp_start, temp_end));
        current = temp_end +1;
    }

    return ranges;
}