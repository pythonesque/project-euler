use math::sieve;
use std::collections::hashmap::HashSet;

euler_problem!(b"73229bab6c5dc1c7cf7a4fa123caf6bc", w, {
    static max: uint = 1_000_000;
    let primes: Vec<uint> = sieve(max).collect();
    // Sums of primes up to this point.
    // Key is that sum(primes starting with m, run length n) = sums[n] - sums[m]
    let mut sums = Vec::with_capacity(primes.len());
    {
        let mut sum = 0;
        for prime in primes.iter() {
            sums.push(sum);
            sum += *prime;
        }
    }
    let len = primes.len();
    let ub = sums.iter().position( |&i| i > *(primes.last().unwrap()) ).unwrap_or(len);
    let primes: HashSet<uint> = primes.move_iter().collect();
    let mut max_run = (0, 0);
    'a: for run in range(1, ub) {
        for (i, sum) in sums.iter().take(len - run).enumerate() {
            let p = sums[i + run] - *sum;
            if primes.contains(&p) {
                max_run = (run, p);
                continue 'a;
            }
        }
    }
    write!(w, "{}", max_run.val1())
})
