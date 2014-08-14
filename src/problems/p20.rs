use digit::{Digits, Base10};
use problems::factorial;
use std::iter::AdditiveIterator;

pub fn run() {
    static n: u32 = 100;
    let digits: Digits<Base10> = factorial(n);
    let sum = digits.get_ref().iter()
        .map( |digit| digit.value() as u32 )
        .sum();
    println!("{}", sum);
}
