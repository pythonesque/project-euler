use math::Digits;
use std::iter::{range_step_inclusive, AdditiveIterator};
use std::num::pow;

euler_problem!(b"a420384997c8a1a93d5a84046117c2aa", w, {
    // For an m digit integer to keep the same digits through 6 multiplications, it must be less
    // than 10 / 6 * 10^(m - 1) = 5 / 3 * 10^(m - 1) => first digit is 1, and all other digits are
    // <= 6.
    static base: u8 = 10;
    static min: u32 = 125874;
    /// precondition: 1 < mult < base
    static mult: u8 = 6;
    static first: u8 = 1;
    static max_after_first: u8 = 6;

    let mut n = min;
    let mut mag = min.digits(base).count() - 1;
    fn compute_max(mag: uint) -> u32 {
        first as u32 * pow(base as u32, mag)
            + range(0u, mag).map( |n| max_after_first as u32 * pow(base as u32, n) ).sum()
    }
    let mut max_mag = compute_max(mag);
    let mut digits = Vec::from_elem(mag + 1, 0u8);
    let mut other = Vec::from_elem(mag + 1, 0u8);
    loop {
        if n > max_mag {
            mag += 1;
            n = pow(base as u32, mag);
            max_mag = compute_max(mag);
            digits.push(0u8);
            other.push(0u8);
        }
        // Because of the above, we know that n and n * 6 have the same number of digits.
        move_iterator!(digits, n.digits(base));
        digits.sort();
        'a: loop {
            for i in range_step_inclusive(n * 2, n * mult as u32, n) {
                move_iterator!(other, i.digits(base));
                other.sort();
                if digits != other { break 'a }
            }
            return write!(w, "{}", n)
        }
        n += 1;
    }
})
