use math::Digits;
use std::iter::{count, MultiplicativeIterator};
use std::num::pow;

euler_problem!(b"6f3ef77ac0e3619e98159e9b6febf557", w, {
    static base: u8 = 10;
    static max: uint = 7;
    let prod = range(0, max)
        .map( |n| pow(base as u32, n) )
        .filter_map( |i|
            count(1u32, 1).flat_map( |n| {
                let vec: Vec<u8> = n.digits(base).collect();
                vec.move_iter().rev()
            }).nth(i as uint - 1)
        ).map( |n| n as u32 )
        .product();
    write!(w, "{}", prod)
})
