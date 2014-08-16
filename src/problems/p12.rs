use math::sieve;
use std::collections::hashmap::HashMap;

fn triangular(n: u32) -> u32 {
    n * (n + 1) / 2
}

euler_problem!(b"8091de7d285989bbfa9a2f9f3bdcc7c0", w, {
    // Triangular numbers are just those equal to n(n+1)/2.  n and n+1 are always relative primes.
    // Counting divisors: by Fundamental Theorem of Arithmetic, every natural number can be uniquely
    // identified as a product of primes up to reordering of the primes.  The number of divisors is
    // the number of possible ways to choose without replacement from a set of p not necessarily
    // distinct elements (where p is the number of prime factors).  This is a problem that a heavy
    // theory attack is extremely well-suited for, but for now we just brute force it.
    static max_prime: uint = 100_000; // guess
    let primes: Vec<uint> = sieve(max_prime).collect();
    for n in range(1, max_prime) {
        let tri = triangular(n as u32);
        let mut factors: HashMap<uint, u32> = HashMap::new();
        let mut temp = tri;
        for ref prime in primes.iter().take_while( |prime| **prime <= n + 1 && temp != 1) {
            while temp % **prime as u32 == 0 {
                temp /= **prime as u32;
                factors.insert_or_update_with(**prime, 1, |_, count| *count += 1 );
            }
        }
        if temp != 1 {
            fail!("Sieve needs to be bigger.")
        }
        // Consider factors as slots, each of which can take on count + 1 values (the powers of
        // the prime factors).  Then straight multiplication should determine the total number of
        // factors.
        let num_factors = factors.iter().fold(1, |num_factors, (_, &exp)| num_factors * (exp + 1) );
        if num_factors > 500 {
            try!(write!(w, "{}", tri));
            break;
        }
    }
    Ok(())
})
