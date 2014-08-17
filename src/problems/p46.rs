use math::sieve;
use std::collections::hashmap::HashSet;

euler_problem!(b"89abe98de6071178edb1b28901a8f459", w, {
    static max: uint = 10_000; // guess
    let primes: Vec<uint> = sieve(max).collect();
    let squares: Vec<uint> = range(0, (max as f64).sqrt() as uint).map( |n| 2 * n * n ).collect();
    let mut odd_composites = primes
        .as_slice()
        .windows(2)
        .flat_map( |window| range(window[0] + 1, window[1]) ) // composite
        .filter( |n| n % 2 == 1 ); // odd
    let primes: HashSet<uint> = primes.iter().map( |p| *p ).collect();
    'a: for c in odd_composites {
        for s in squares.iter().take_while( |& &s| s <= c) {
            if primes.contains(&(c - *s)) { continue 'a }
        }
        try!(write!(w, "{}", c));
        return Ok(());
    }
    Ok(())
})
