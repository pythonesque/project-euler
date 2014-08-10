use std::collections::bitv::Bitv;
use std::iter::range_step;

// Sieve of Eratosthenes
pub struct Sieve {
    min: uint,
    max: uint,
    sieve: Bitv,
}

pub fn sieve(max: uint) -> Sieve {
    Sieve {
        min: 0u,
        max: (max as f64).sqrt() as uint, // rounds down
        sieve: Bitv::with_capacity(max - 1, true),
    }
}

impl Iterator<uint> for Sieve {
    fn next(&mut self) -> Option<uint> {
        for i in range(self.min, self.max) {
            if self.sieve[i] {
                self.min = i + 1;
                let prime = self.min + 1;
                for j in range_step(prime * prime - 2, self.sieve.len(), prime) {
                    self.sieve.set(j, false);
                }
                return Some(prime)
            }
        }
        for i in range(self.max, self.sieve.len()) {
            if self.sieve[i] {
                self.max = i + 1;
                let prime = self.max + 1;
                return Some(prime)
            }
        }
        None
    }
}

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
