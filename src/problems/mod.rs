use std::fmt;
use std::collections::bitv::Bitv;
use std::collections::hashmap::{HashSet};
use std::iter::{range_step};
use std::num::{One, pow, Zero};

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
pub mod p21;
pub mod p22;
pub mod p23;
pub mod p24;
pub mod p25;
pub mod p26;
pub mod p27;
pub mod p67;

// Sieve of Eratosthenes
pub struct Sieve {
    min: uint,
    max: uint,
    sieve: Bitv,
}

pub fn quicksort<'a, T: Clone + Ord>(array: &'a mut [T]) {
    let n = array.len();
    if n < 2 { return }
    let p = {
        // choose pivot
        let pivot_index = (n - 1) / 2;
        // partition
        let pivot_value = array[pivot_index].clone();
        array.swap(pivot_index, n - 1);
        let mut store_index = 0;
        for i in range(0, n - 1) {
            if array[i] < pivot_value {
                array.swap(i, store_index);
                store_index += 1;
            }
        }
        array.swap(store_index, n - 1);
        store_index
    };
    if p > 0 { quicksort(array.mut_slice(0, p)) }
    if p + 1 < n { quicksort(array.mut_slice(p + 1, n)) }

}

pub fn sieve(max: uint) -> Sieve {
    Sieve {
        min: 0u,
        max: if max < 2 { 0 } else { (max as f64).sqrt() as uint } , // rounds down
        sieve: Bitv::with_capacity(if max == 0 { 0 } else { max - 1 }, true),
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

pub fn factor<'a, T: Iterator<&'a uint>>(n: uint, primes: T) -> HashSet<uint> {
    let max_prime = n / 2;
    let mut num_factors = 1;
    let prime_factors: Vec<(uint, uint)> = primes
        .take_while( |& &p| p <= max_prime)
        .filter_map( |&p| {
            let mut n = n;
            let mut q = 1;
            while n % p == 0 {
                q += 1;
                n /= p;
            }
            if q == 1 {
                None
            } else {
                num_factors *= q;
                Some((p, q))
            }
        })
        .collect();
    let mut factors: HashSet<uint> = range(0, num_factors)
        .map( |i| prime_factors.iter()
            .fold( (1, i), |(fact, i), &(p, q)| (fact * pow(p, i % q), i / q) ).val0() )
        .collect();
    factors.insert(n);
    factors
}
