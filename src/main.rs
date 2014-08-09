use problems::{p1, p2};
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
                _ => println!("Invalid problem number."),
            },
            _ => println!("Not a number."),
        }
    }
}
