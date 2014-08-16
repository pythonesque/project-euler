use digit::{Digits, Base10};
use std::iter::AdditiveIterator;
use std::num::from_u32;

euler_problem!(b"27a1779a8a8c323a307ac8a70bc4489d", w, {
    // Cannot be written as sum of 5th powers if n * 9^5 < 10^n
    // log(n * 9^5) < n
    // log(n) + 5log(9) < n
    // n - log(n) > 5log(9)
    // => n < 6
    // < 6 digits -- we can use u32
    static max: u32 = 1_000_000;
    let sum = range(2, max)
        .filter( |&n| {
            let digits: Digits<Base10> = from_u32(n).unwrap();
            let digits = digits.get_ref();
            let sum_pow_5s = digits.iter().map( |&digit| {
                let digit = digit.value();
                let square = digit as u32 * digit as u32;
                let quad = square * square;
                quad * digit as u32
            }).sum();
            let sum = digits.iter().fold((1, 0), |(dec, sum), n|
                (dec * 10, sum + dec * n.value() as u32) ).val1();
            sum == sum_pow_5s
        }).sum();
    write!(w, "{}", sum)
})
