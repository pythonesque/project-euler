use std::iter::AdditiveIterator;
use problems::sieve;

pub fn run() {
    static max: uint = 2_000_000;
    let sum_primes = sieve(max - 1).sum();
    println!("{}", sum_primes);
}
