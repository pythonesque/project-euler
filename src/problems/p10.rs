use std::iter::AdditiveIterator;
use math::sieve;

euler_problem!(b"d915b2a9ac8749a6b837404815f1ae25", w, {
    static max: uint = 2_000_000;
    let sum_primes = sieve(max - 1).sum();
    write!(w, "{}", sum_primes)
})
