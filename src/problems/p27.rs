use problems::sieve;
use std::collections::hashmap::HashSet;
use std::num::abs;

pub fn run() {
    static coeff: uint = 1000;
    static prime_ub: uint = 10_000; // Guess
    let mut max = (0, 0, 0);
    let primes: HashSet<uint> = sieve(prime_ub).collect();
    let primes_to_abs_coeff: Vec<uint> = sieve(coeff - 1).collect();
    // b must itself be a prime < 1000, since for n = 0, 0^2 + a0 + b must be prime.
    // And a + b + 1 must be prime, since for n = 1, 1^2 + a1 + b must be prime.
    for m in primes_to_abs_coeff.iter().take_while( |& &p| p < coeff ) {
        let mut iter = primes_to_abs_coeff.iter().filter_map( |&b| {
            let a = *m as int - b as int - 1;
            if abs(a) < coeff as int { Some((a, b as int)) } else { None }
        });
        for (a, b) in iter {
            let mut n = 0;
            loop {
                let p = n * n + a * n + b;
                if !primes.contains(&(p as uint)) { break }
                n += 1;
            }
            if max.val2() < n {
                max = (a, b, n);
            }
        }
    }
    println!("{}", max.val0() * max.val1());
}
