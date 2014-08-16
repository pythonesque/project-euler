use digit::{Digits, Base10};
use std::collections::hashmap::HashSet;
use std::iter::range_inclusive;
use std::num::from_u8;

pub fn run() {
    static max: u8 = 100;
    let mut set = HashSet::new();
    for a in range_inclusive(2, max) {
        let a: Digits<Base10> = from_u8(a).unwrap();
        let mut c = a.clone();
        for b in range_inclusive(2, max) {
            c = c * a;
            set.insert(c.clone());
        }
    }
    println!("{}", set.len());
}
