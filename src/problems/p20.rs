use digit::{Digits, Base10};
use math::factorial;
use std::iter::AdditiveIterator;

euler_problem!(b"443cb001c138b2561a0d90720d6ce111", w, {
    static n: u32 = 100;
    let digits: Digits<Base10> = factorial(n);
    let sum = digits.get_ref().iter()
        .map( |digit| digit.value() as u32 )
        .sum();
    write!(w, "{}", sum)
})
