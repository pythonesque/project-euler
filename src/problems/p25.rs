use digit::{Digits, Base10};
use std::mem::swap;
use std::num::One;

pub fn run() {
    let mut fib_i: Digits<Base10> = One::one();
    let mut fib_j: Digits<Base10> = One::one();
    let mut i = 2u32;
    while fib_j.get_ref().len() < 1000 {
        swap(&mut fib_i, &mut fib_j);
        fib_j.add_in_place(&fib_i);
        i += 1;
    }
    println!("{}", i);
}
