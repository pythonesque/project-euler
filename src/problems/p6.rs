use std::iter::{range_inclusive, AdditiveIterator};

pub fn run() {
    static max: i64 = 100;
    let sum_squares = range_inclusive(1, max).map( |x| x * x).sum();
    let range_sum = max * (max + 1) / 2;
    let square_sum = range_sum * range_sum;
    println!("{}", square_sum - sum_squares);
}
