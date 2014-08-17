use math::Digits;
use std::collections::bitv::Bitv;
use std::collections::bitv::BitvSet;
use std::iter::{AdditiveIterator, range_inclusive};
use std::num::pow;

euler_problem!(b"100f6e37d0b0564490a2ee27eff0660d", w, {
    static max: u8 = 9;
    /// precondition: max < base
    static base: u8 = 10;
    // a * b = c
    // a.len() + b.len() + c.len() == max
    // a.len() + b.len() - 1 <= c.len() <= a.len() + b.len()
    // a.len() + b.len() + c.len() - 1 <= 2 * c.len() <= a.len() + b.len() + c.len()
    // max - 1 <= 2 * c.len() <= max
    // (max - 1) / 2 <= c.len <= max / 2
    static min_digits: u8 = (max - 1) / 2;
    static max_digits: u8 = max / 2;
    let min_product = pow(base as u32, min_digits as uint - 1);
    let max_product = pow(base as u32, max_digits as uint) - 1;
    let pandigital = BitvSet::from_bitv(Bitv::with_capacity(max as uint, true));
    let mut digits = BitvSet::with_capacity(max as uint);
    let mut digit_vec = [0u8, .. (max - min_digits) as uint];
    let sum = range_inclusive(min_product, max_product)
        .filter( |&n| {
            digits.clear();
            for digit in n.digits(base) {
                if digit == 0 || max < digit || digits.contains(&(digit as uint - 1)) {
                    return false;
                }
                digits.insert(digit as uint - 1);
            }
            digits.symmetric_difference_with(&pandigital);
            for (digit, other) in digits.iter().zip(digit_vec.mut_iter()) {
                *other = digit as u8 + 1;
            }
            let len = digits.len();
            for permutation in digit_vec.mut_slice_to(len).permutations() {
                for i in range(1, len) {
                    let (left, right) = permutation.as_slice().split_at(i);
                    let is_pandigital = Digits::from_digits(left.iter(), base).and_then( |a: u32|
                        Digits::from_digits(right.iter(), base).filtered( |&b: &u32|
                        a * b == n ) ).is_some();
                    if is_pandigital { return true }
                }
            }
            false
        }).sum();
    write!(w, "{}", sum)
})
