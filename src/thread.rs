use std::thread;
use crate::compute::find_prime;

pub fn make_threads (n_thread: u64, n_range: Vec<u64>) {
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
            
            for (ix, (prime, timestamp)) in primes.iter().enumerate() {
                println!("Prime {}: {}, Timestamp: {:?}", ix + 1, prime, timestamp);
            }
            println!();
        }
    }
}

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