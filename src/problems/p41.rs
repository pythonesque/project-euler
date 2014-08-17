use math::{Digits, sieve};
use std::num::pow;

euler_problem!(b"d0a1bd6ab4229b2d0754be8923431404", w, {
    static max: u8 = 9;
    /// precondition: max < base
    static base: u8 = 10;
    let max_prime = (pow(base as uint, max as uint) as f64).sqrt() as uint;
    let primes: Vec<uint> = sieve(max_prime).collect();
    let mut digits = [0u8, ..max as uint];
    let mut digits = digits.as_mut_slice();
    for (i, digit) in digits.mut_iter().rev().enumerate() {
        *digit = i as u8 + 1;
    }
    'a: loop {
        loop {
            let p: u32 = match Digits::from_digits(digits.iter().rev(), base) {
                Some(p) => p,
                None => break
            };
            if primes.iter().take_while( |& &q| q as u32 <= p ).all( |&q| p % q as u32 != 0 ) {
                break 'a;
            }
            if !digits.prev_permutation() { break }
        }
        digits.mut_pop_ref();
        digits.reverse();
        if digits.len() == 0 { break }
    }
    let max_p: u32 = Digits::from_digits(digits.iter().rev(), base).unwrap();
    write!(w, "{}", max_p)
})
