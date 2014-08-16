use std::iter::{range_inclusive, AdditiveIterator};

euler_problem!(b"867380888952c39a131fe1d832246ecc", w, {
    static max: i64 = 100;
    let sum_squares = range_inclusive(1, max).map( |x| x * x).sum();
    let range_sum = max * (max + 1) / 2;
    let square_sum = range_sum * range_sum;
    write!(w, "{}", square_sum - sum_squares)
})
