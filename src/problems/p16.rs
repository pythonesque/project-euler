use std::iter::AdditiveIterator;
use digit::{double_digits, Digit, Digits};

euler_problem!(b"6a5889bb0190d0211a991f47bb19a777", w, {
    static power: uint = 1000;
    let mut digits = Digits::make(vec!(Digit::make(1)));
    for i in range(0, power) {
        double_digits(&mut digits);
    }
    let digit_sum = digits.get_ref().iter().map(|n| n.value() as u16).sum();
    write!(w, "{}", digit_sum)
})
