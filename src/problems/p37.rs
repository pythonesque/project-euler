use math::{sieve, Digits};
use std::collections::hashmap::HashSet;
use std::iter::AdditiveIterator;

euler_problem!(b"cace46c61b00de1b60874936a093981d", w, {
    static base: u8 = 10;
    static count: uint = 11; // Guess
    static max: uint = 1_000_000;
    let primes: HashSet<uint> = sieve(max).collect();
    let sum = primes.iter()
        .filter( |& &p| {
            if p < base as uint { return false }
            let digits: Vec<u8> = p.digits(base).collect();
            let len = digits.len();
            range(1, len).all( |n|
                Digits::from_digits(digits.iter().take(n), base)
                .filtered( |q| primes.contains(q) )
                .is_some() ) &&
            range(1, len).all( |n|
                Digits::from_digits(digits.iter().skip(n), base)
                .filtered( |q| primes.contains(q) )
                .is_some() )
        }).take(count)
        .map( |&p| p )
        .sum();
    write!(w, "{}", sum)
})
