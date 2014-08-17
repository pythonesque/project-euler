use math::{Digits, sieve};
use std::num::pow;

euler_problem!(b"115253b7721af0fdff25cd391dfc70cf", w, {
    static min: u8 = 0;
    static max: u8 = 9;
    static size: u8 = max - min + 1;
    static chunk: u8 = 3;
    /// precondition: max < base
    static base: u8 = 10;
    let max_prime = pow(base as uint, chunk as uint) as uint;
    let primes: Vec<uint> = sieve(max_prime).take((size - chunk) as uint).collect();
    let mut digits = [0u8, .. size as uint];
    let digits = digits.as_mut_slice();
    for (i, digit) in digits.mut_iter().enumerate() {
        *digit = i as u8 + min;
    }
    let mut sum = 0u64;
    loop {
        'a: loop {
            let shift_digits = digits.slice_from(1);
            for (chunk_digits, &p) in shift_digits.windows(chunk as uint).zip(primes.iter()) {
                let c: u16 = match Digits::from_digits(chunk_digits.iter().rev(), base) {
                    Some(c) => c,
                    None => break 'a
                };
                if c % p as u16 != 0 { break 'a }
            }
            match Digits::from_digits(digits.iter().rev(), base) {
                Some(n) => sum += n,
                None => (),
            };
            break
        }
        if !digits.next_permutation() { break }
    }
    write!(w, "{}", sum)
})
