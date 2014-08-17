use math::Digits;
use std::iter::AdditiveIterator;

euler_problem!(b"0e175dc2f28833885f62e7345addff03", w, {
    static b1: u8 = 10;
    static b2: u8 = 2;
    static max: uint = 1_000_000;
    let sum = range(1, max)
        .filter( |n| {
            let v1: Vec<u8> = n.digits(b1).collect();
            if !v1.iter().zip(v1.iter().rev()).all( |(&d,&rd)| d == rd ) {
                return false;
            }
            let v2: Vec<u8> = n.digits(b2).collect();
            v2.iter().zip(v2.iter().rev()).all( |(&d,&rd)| d == rd )
        }).sum();
    write!(w, "{}", sum)
})
