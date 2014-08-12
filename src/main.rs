extern crate num;

use problems::{
    p1,
    p2,
    p3,
    p4,
    p5,
    p6,
    p7,
    p8,
    p9,
    p10,
    p11,
    p12,
    p13,
    p14,
    p15,
    p16,
    p17,
    p18,
};
use std::os;

mod problems;

fn main() {
    let args = os::args();
    let mut iter = args.iter();
    iter.next();
    for arg in iter {
        match from_str::<i32>(arg.as_slice()) {
            Some(n) => match n {
                1 => p1::run(),
                2 => p2::run(),
                3 => p3::run(),
                4 => p4::run(),
                5 => p5::run(),
                6 => p6::run(),
                7 => p7::run(),
                8 => p8::run(),
                9 => p9::run(),
                10 => p10::run(),
                11 => p11::run(),
                12 => p12::run(),
                13 => p13::run(),
                14 => p14::run(),
                15 => p15::run(),
                16 => p16::run(),
                17 => p17::run(),
                18 => p18::run(),
                _ => println!("Invalid problem number."),
            },
            _ => println!("Not a number."),
        }
    }
}
