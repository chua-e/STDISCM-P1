use std::thread;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use crate::compute::{PrimeTime, is_prime, has_divisor};
use chrono::{DateTime, Utc};

/**
 * > A1B1
 * Each thread gets a an allocated range from function 'allocate_ranges()'
 * and each thread gets the start and end of an allocated range to find
 * all the prime numbers in each, once a prime is found the thread prints 
 * the thread number, prime number, and timestamp immediately
 * 
 * 
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
}

// print immediately, linear search
/**
 * > A2B1
 * Each thread is tasked with finding a prime for one number, each
 * thread is allocated a number within the defined config range 
 */
pub fn variant_2(n_thread: u64, n_range: Vec<u64>) {
    // let (start, end) = (n_range[0], n_range[1]);
    // let (tx, rx) = mpsc::channel::<(usize, u64, DateTime<Utc>)>();
    // let next = Arc::new(AtomicU64::new(start));

    // for thread_id in 1..=n_thread {
    //     let tx = tx.clone();
    //     let next = Arc::clone(&next);

    //     thread::spawn(move || loop {
    //         let n = next.fetch_add(1, Ordering::Relaxed);
    //         if n > end { break; } 

    //         if is_prime(n) {
    //             thread::sleep(std::time::Duration::from_millis(10));
    //             tx.send((thread_id.try_into().unwrap(), n, Utc::now())).unwrap();
    //         }
    //     });
    // }
    
    // drop(tx);

    // for (id, prime, timestamp) in rx {
    //     println!("Thread {}: Prime {}, Timestamp: {:?}", id, prime, timestamp);
    // }

    linear_search(n_thread, n_range[0], n_range[1], |n| {
        println!("Prime {}, Timestamp: {:?}", n, Utc::now());
    });
}

// wait to print, straight division
pub fn variant_3 (n_thread: u64, n_range: Vec<u64>) {
    let range_list = allocate_ranges(n_thread, n_range);
    let mut thread_list = Vec::new();

    for (i, (start, end)) in range_list.into_iter().enumerate() {
        let thread_id = i + 1;
        let t = thread::spawn(move || {
        //    let primes = find_prime(&start, &end);
        //     (thread_id, start, end, primes)
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
            println!("Thread {}", id);
            
            for (ix, (prime, _)) in primes.iter().enumerate() {
                println!("Prime {}: {}", ix + 1, prime);
            }
            println!();
        }
    }
}

// wait to print, linear search
pub fn variant_4(n_thread: u64, n_range: Vec<u64>) {
    let mut primes: PrimeTime = Vec::new();
    linear_search(n_thread, n_range[0], n_range[1], |n| primes.push((n, Utc::now())));
    for (ix, (prime, ts)) in primes.iter().enumerate() {
        println!("Prime {}: {}, Timestamp: {:?}", ix + 1, prime, ts);
    }
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

/**
 * 
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
        let total = limit-1;
        let chunk = total.div_ceil(n_thread);

        let mut sent =0;
        for (k, job_tx) in job_txs.iter().enumerate() {
            let lo = 2 + k as u64 * chunk;
            if lo > limit { break; }
            let hi = (lo + chunk - 1).min(limit);
            job_tx.send((n, lo, hi)).unwrap();
            sent += 1;
        }

        let mut composite = false;
        for _ in 0..sent {
            let (_id, found) = res_rx.recv().unwrap();
            composite |= found;
        }
        if !composite { on_prime(n); }
    }

    drop(job_txs);
    for h in handles { h.join().unwrap(); 

    }
}