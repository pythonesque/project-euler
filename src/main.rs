extern crate num;

use std::os;

mod digit;
mod problems;

fn main() {
    let args = os::args();
    let mut iter = args.iter();
    iter.next();
    for arg in iter {
        match from_str::<i32>(arg.as_slice()) {
            Some(n) => match n {
                1 => problems::p1::run(),
                2 => problems::p2::run(),
                3 => problems::p3::run(),
                4 => problems::p4::run(),
                5 => problems::p5::run(),
                6 => problems::p6::run(),
                7 => problems::p7::run(),
                8 => problems::p8::run(),
                9 => problems::p9::run(),
                10 => problems::p10::run(),
                11 => problems::p11::run(),
                12 => problems::p12::run(),
                13 => problems::p13::run(),
                14 => problems::p14::run(),
                15 => problems::p15::run(),
                16 => problems::p16::run(),
                17 => problems::p17::run(),
                18 => problems::p18::run(),
                19 => problems::p19::run(),
                20 => problems::p20::run(),
                21 => problems::p21::run(),
                22 => problems::p22::run(),
                23 => problems::p23::run(),
                24 => problems::p24::run(),
                67 => problems::p67::run(),
                _ => println!("Invalid problem number."),
            },
            _ => println!("Not a number."),
        }
    }
}
