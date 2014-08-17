use math::Digits;
use num::bigint::BigUint;
use std::iter::range_inclusive;
use std::num::{one, pow, zero};

euler_problem!(b"0829124724747ae1c65da8cae5263346", w, {
    static max: uint = 1000;
    static take_digits: uint = 10;
    static base: u8 = 10;
    let one_cache = one();
    let mut sum: BigUint = zero();
    let mut power: BigUint = one();
    for i in range_inclusive(1, max) {
        let exp = power;
        power = exp + one_cache;
        sum = sum + pow(exp, i);
    }
    let digits: Vec<u8> = sum.digits(base).take(take_digits).collect();
    let last: u64 = Digits::from_digits(digits.iter(), base).unwrap();
    write!(w, "{}", last)
})
