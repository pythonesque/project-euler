use std::iter::AdditiveIterator;
use digit::{double_digits, Digit, Digits};

pub fn run() {
    static power: uint = 1000;
    let mut digits = Digits::make(vec!(Digit::make(1)));
    for i in range(0, power) {
        double_digits(&mut digits);
    }
    let digit_sum = digits.get_ref().iter().map(|n| n.value() as u16).sum();
    println!("{}", digit_sum);
}
