use math::{Digits, factorial};
use std::iter::AdditiveIterator;
use std::num::pow;

euler_problem!(b"60803ea798a0c0dfb7f36397d8d4d772", w, {
    // Let n be an m-digit number, and d_0 ... d_{m-1} be its digits from right to left.
    // Then n = Sum{0 <= i < m}(d_i * 10^i), so 10^{m - 1} - 1 < n.
    // Let f(n) = Sum{0 <= i < m}(d_i!), so f(n) <= m * 9!.
    // Thus, m * 9! <= 10^{m - 1} - 1 => f(n) < n.
    // The intersection occurs between m = 7 and m = 8, so m is at most 7.
    static base: u8 = 10;
    static max_digits: uint = 7;
    let mut factorials = [0u32, .. base as uint];
    for (store, fact) in factorials.mut_iter().zip(factorial::<u32>()) {
        *store = fact;
    }
    let sum = range(3, pow(base as u32, max_digits))
        .filter( |&n| n == n.digits(base).map( |digit| factorials[digit as uint] ).sum() )
        .sum();
    write!(w, "{}", sum)
})
