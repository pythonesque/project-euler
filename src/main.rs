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
                _ => println!("Invalid problem number."),
            },
            _ => println!("Not a number."),
        }
    }
}
