use std::thread;
use std::sync::{mpsc};
use crate::compute::{PrimeTime, is_prime, has_divisor};
use chrono::{DateTime, Utc};

/**
 * > A1B1: Print immediately + Straight division
 * Each thread gets a an allocated range from function 'allocate_ranges()'
 * and each thread gets the start and end of an allocated range to find
 * all the prime numbers in each, once a prime is found the thread prints 
 * the thread number, prime number, and timestamp immediately
 */
pub fn variant_1(n_thread: u64, n_range: Vec<u64>, will_print: bool) {
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

    if will_print {
        for (id, prime, timestamp) in rx {
            println!("Thread {}: Prime {}, Timestamp: {:?}", id, prime, timestamp);
        }
    }
}

/**
 * > A2B1: Print immediately + Linear search
 * Using the linear_search() function, when a prime n is found, it is 
 * printed immediately for this variant
 */
pub fn variant_2(n_thread: u64, n_range: Vec<u64>, will_print: bool) {
    linear_search(n_thread, n_range[0], n_range[1], |n| {
        if will_print {
            println!("Prime {}, Timestamp: {:?}", n, Utc::now());
        }
    });
}

/**
 * > A1B2: Wait to print + Straight division
 * Each thread gets a an allocated range from function 'allocate_ranges()'
 * and each thread gets the start and end of an allocated range to find
 * all the prime numbers in each, once a prime is found it is stored 
 * in a primes vector and is then printed per thread after all threads
 * finish working
 */
pub fn variant_3 (n_thread: u64, n_range: Vec<u64>, will_print: bool) {
    let range_list = allocate_ranges(n_thread, n_range);
    let mut thread_list = Vec::new();

    for (i, (start, end)) in range_list.into_iter().enumerate() {
        let thread_id = i + 1;
        let t = thread::spawn(move || {
            let mut primes: PrimeTime = Vec::new();

            for n in start..=end {
                if is_prime(n) {
                    primes.push((n, Utc::now()));
                }
            }
            (thread_id, primes)
        });

        thread_list.push(t);
    }
    
    for t in thread_list {
        if let Ok((id, primes)) = t.join() {
            if will_print {
                println!("Thread {}", id);
            
                for (ix, (prime, _)) in primes.iter().enumerate() {
                    println!("Prime {}: {}", ix + 1, prime);
                }
                println!();
            }
        }
    }
}

/**
 * > A2B2: Wait to print + Linear search
 * Using the linear_search() function, when a prime n is found, it is 
 * pushed into a primes vector and printed once all threads finish
 */
pub fn variant_4(n_thread: u64, n_range: Vec<u64>, will_print: bool) {
    let mut primes: PrimeTime = Vec::new();
    linear_search(n_thread, n_range[0], n_range[1], |n| primes.push((n, Utc::now())));
    for (ix, (prime, ts)) in primes.iter().enumerate() {
        if will_print {
            println!("Prime {}: {}, Timestamp: {:?}", ix + 1, prime, ts);
        }
    }
}

/**
 * Helper function that allocates numbers from the config range
 * into equal segments per thread, only returns the first and
 * last number of the segmented range. If a range is not equally 
 * divisible by n_threads, the last thread more than likely gets
 * less numbers in its range then the other threads
 */
fn allocate_ranges (n_thread: u64, n_range: Vec<u64>) -> Vec<(u64, u64)> {
    if n_range.len() < 2 || n_thread == 0 {
        return Vec::new();
    }

    let range_start = n_range[0];
    let range_end = n_range[1];

    let total = range_end - range_start + 1;
    let n = n_thread.min(total);
    let base = total / n;
    let extra = total % n; 

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let mut current = range_start;
    for i in 0..n {
        let increment = base + if i < extra { 1 } else { 0 };
        ranges.push((current, current + increment - 1));
        current += increment;
    }

    return ranges;
}

/**
 * This is another helper function but for linear search. It runs from the
 * start to the end of the given range in the config and splits the divisibility
 * testing for each thread defined in n_thread. It splits the divisor widow among
 * threads using the allocate_ranges() function and each thread checks its own
 * allocated range for a divisor.
 * 
 * The on_prime parameter is used as a closure to determine what to do with a certain
 * value if it is prime depending on the variant.
 */
fn linear_search (n_thread: u64, start: u64, end: u64, mut on_prime: impl FnMut(u64)) {
    let (res_tx, res_rx) = mpsc::channel::<(u64, bool)>();
    let mut job_txs= Vec::new();
    let mut handles = Vec::new();

    for id in 1..=n_thread {
        let (job_tx, job_rx) =mpsc::channel::<(u64, u64, u64)>();
        let res_tx = res_tx.clone();

        handles.push(thread::spawn(move || {
            for (n, low, high) in job_rx{
                res_tx.send((id, has_divisor(n, low, high))).unwrap();
            }
        }));
        job_txs.push(job_tx);
    }

    drop(res_tx);

    for n in start..=end {
        if n <= 1 || (n > 2 && n % 2 == 0) {
            continue;
        }

        if n <= 3 {
            on_prime(n);
            continue;
        }

        let limit = n.isqrt();
        let slices = allocate_ranges(n_thread, vec![2, limit]);

        for (job_tx, (low, high)) in job_txs.iter().zip(slices.iter()) {
            job_tx.send((n, *low, *high)).unwrap();
        }

        let mut composite = false;
        for _ in 0..slices.len() {
            let (_id, found) = res_rx.recv().unwrap();
            composite |= found;
        }

        if !composite {
            on_prime(n);
        }
    }

    drop(job_txs);
    for h in handles { 
        h.join().unwrap(); 
    }
}