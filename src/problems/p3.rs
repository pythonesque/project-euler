use math::sieve;

euler_problem!(b"94c4dd41f9dddce696557d3717d98d82", w, {
    let mut n = 600_851_475_143;
    let max_prime_factor = (n as f64).sqrt() as uint;
    for prime in sieve(max_prime_factor) {
        while n % prime == 0 {
            n /= prime;
        }
        if n == 1 {
            try!(write!(w, "{}", prime))
            break;
        }
    }
    Ok(())
})
