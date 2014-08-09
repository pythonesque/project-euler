use problems::p1;
use std::os;

mod problems;

fn main() {
    let args = os::args();
    let mut iter = args.iter();
    iter.next();
    for arg in iter {
        match from_str::<i32>(arg.as_slice()) {
            Some(1) => p1::run(),
            _ => println!("Invalid problem number."),
        }
    }
}
