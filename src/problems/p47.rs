use math::{sieve, prime_factors};
use std::iter::count;

euler_problem!(b"748f517ecdc29106e2738f88aa7530f4", w, {
    static max: uint = 10_000; // Guess
    static chunk: uint = 4;
    let primes: Vec<uint> = sieve(max).collect();
    let mut run = 0;
    for n in count(1, 1) {
        let pf: Vec<uint> = prime_factors(n, primes.iter());
        if pf.len() == chunk { run += 1 } else { run = 0 }
        if run == chunk {
            try!(write!(w, "{}", n - chunk + 1));
            return Ok(())
        }
    }
    Ok(())
})
