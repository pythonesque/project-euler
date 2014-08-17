use num::bigint::BigUint;
use std::collections::bitv::Bitv;
use std::collections::hashmap::HashSet;
use std::fmt;
use std::iter::{range_step, Unfold};
use std::num::{one, One, pow, zero, Zero};

// Sieve of Eratosthenes
pub struct Sieve {
    min: uint,
    max: uint,
    sieve: Bitv,
}

pub fn sieve(max: uint) -> Sieve {
    Sieve {
        min: 0u,
        max: if max < 2 { 0 } else { (max as f64).sqrt() as uint } , // rounds down
        sieve: Bitv::with_capacity(if max == 0 { 0 } else { max - 1 }, true),
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
                self.min = self.max;
                let prime = self.max + 1;
                return Some(prime)
            }
        }
        self.max = self.sieve.len();
        None
    }
}

pub fn factorial<'a, T: Clone + Mul<T,T> + One + Zero + fmt::Show>() -> Unfold<'a, T, (Option<T>, T)> {
    let one_: T = one();
    let zero_: T = zero();
    Unfold::new((Some(one_), zero_), |st| {
        let &(ref mut fact, ref mut fact_n) = st;
        *fact_n = *fact_n + one();
        match fact.take() {
            Some(f) => {
                *fact = Some(f * *fact_n);
                Some(f)
            },
            None => None
        }
    })
}

pub fn prime_factors<'a, T: Iterator<&'a uint>, C: FromIterator<uint>>(n: uint, primes: T) -> C {
    primes.take_while( |& &p| p <= n).filter_map( |&p|
        if n % p == 0 { Some(p) } else { None }
    ).collect()
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

/// If num == 0, the iterator will yield no elements.
pub trait Digits: Zero + One + Div<Self, Self> + Mul<Self, Self> + Rem<Self, Self> + FromPrimitive + ToPrimitive {
    fn digits<'a>(self, base: u8) -> Unfold<'a, u8, (Self, Self)> {
        match FromPrimitive::from_u8(base) {
            Some(base) => Unfold::new((self, base), |st| {
                let &(ref mut num, ref base) = st;
                if (*num).is_zero() {
                    None
                } else {
                    let rem = *num % *base;
                    *num = *num / *base;
                    rem.to_u8()
                }
            }),
            None => Unfold::new((zero(), zero()), |_| { None }),
        }
    }

    fn from_digits<'r, T: Iterator<&'r u8>>(mut iter: T, base: u8) -> Option<Self> {
        FromPrimitive::from_u8(base).and_then( |baseconv| {
            let mut sum: Self = zero();
            let mut mag: Self = one();
            for digit in iter {
                if *digit >= base { return None; }
                match FromPrimitive::from_u8(*digit) {
                    Some(digit) => {
                        let prod = mag * digit;
                        sum = sum + prod;
                        mag = mag * baseconv
                    },
                    None => return None
                }
            }
            Some(sum)
        })
    }
}

impl Digits for u16 {}
impl Digits for u32 {}
impl Digits for u64 {}
impl Digits for uint {}
impl Digits for BigUint {}
