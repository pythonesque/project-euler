use problems::sieve;

pub fn run() {
    let mut n = 600_851_475_143;
    let max_prime_factor = (n as f64).sqrt() as uint;
    for prime in sieve(max_prime_factor) {
        if n == 1 {
            println!("{}", prime);
            break
        } else {
            while n % prime == 0 {
                n /= prime;
            }
        }
    }
}
