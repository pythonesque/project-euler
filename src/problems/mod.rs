use std::collections::bitv::Bitv;
use std::fmt;
use std::iter::range_step;
use std::num::{One, Zero};

pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;
pub mod p10;
pub mod p11;
pub mod p12;
pub mod p13;
pub mod p14;
pub mod p15;
pub mod p16;
pub mod p17;
pub mod p18;
pub mod p19;
pub mod p20;
pub mod p67;

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

pub fn factorial<T: Clone + Mul<T,T> + One + Zero + fmt::Show>(mut n: u32) -> T {
    let mut fact: T = One::one();
    let mut fact_n: T = Zero::zero();
    while !n.is_zero() {
        fact_n = fact_n + One::one();
        fact = fact * fact_n;
        n = n - One::one();
    }
    fact
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
                self.min = self.max;
                let prime = self.max + 1;
                return Some(prime)
            }
        }
        self.max = self.sieve.len();
        None
    }
}
