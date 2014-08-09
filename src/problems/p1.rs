use std::iter::AdditiveIterator;

pub fn run() {
    let res = range(0, 1000u)
        .filter( |i| i % 3 == 0 || i % 5 == 0)
        .sum();
    println!("{}", res);
}
