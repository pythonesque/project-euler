use math::{factor, sieve};
use std::collections::{BitvSet, Bitv};
use std::iter::{AdditiveIterator, range_inclusive};

euler_problem!(b"2c8258c0604152962f7787571511cf28", w, {
    static max: uint = 28123;
    let primes: Vec<uint> = sieve(max / 2).collect();
    let abundant: Vec<uint> = range_inclusive(1, max)
        .filter( |&n| factor(n, primes.iter()).move_iter().sum() - n > n )
        .collect();
    let mut non_summable = BitvSet::from_bitv(Bitv::with_capacity(max + 1, true));
    for (index, &i) in abundant.iter().enumerate() {
        for &j in abundant.iter().skip(index) {
            let k = i + j;
            if k > max { break };
            non_summable.remove(&k);
        }
    }
    let sum = non_summable.iter().sum();
    write!(w, "{}", sum)
})
