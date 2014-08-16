use math::{sieve, factor};
use std::iter::{AdditiveIterator, range_inclusive};

euler_problem!(b"51e04cd4e55e7e415bf24de9e1b0f3ff", w, {
    static max: uint = 10_000;
    let primes: Vec<uint> = sieve(max / 2).collect();
    let mut div_sums = [0u, .. max];
    let mut amicable_sum = 0;
    for n in range_inclusive(1, max) {
        let sum = factor(n, primes.iter()).iter().map( |&i| i ).sum() - n;
        if sum > 0 && sum <= max && div_sums[sum - 1] == n {
            amicable_sum += n + sum;
        } else {
            // We only need to do this if we didn't find a partner, since amicable numbers only
            // pair with each other.
            div_sums[n - 1] = sum;
        }
    }
    write!(w, "{}", amicable_sum)
})
