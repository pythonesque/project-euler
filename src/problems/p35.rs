use math::{Digits, sieve};
use std::collections::hashmap::HashSet;

euler_problem!(b"b53b3a3d6ab90ce0268229151c9bde11", w, {
    static base: u8 = 10;
    static max: uint = 1_000_000;
    let primes: HashSet<uint> = sieve(max).collect();
    let count = primes.iter()
        .filter( |p| {
            let mut vec: Vec<u8> = p.digits(base).collect();
            let len = vec.len();
            vec.extend(p.digits(base).take(len - 1));
            vec.as_slice().windows(len).all( |window|
                Digits::from_digits(window.iter(), base)
                    .filtered( |q| primes.contains(q) )
                    .is_some() )
        })
        .count();
    write!(w, "{}", count)
})
