#![feature(default_type_params)]
#![feature(macro_rules)]

extern crate libc;
extern crate num;

mod digit;
mod io;
mod math;
mod problems;
mod sort;

#[cfg(not(test))]
pub mod main {
    use std::io::{MemWriter, stderr};
    use std::os;
    use std::str;
    use problems = problems::main;
    use problems::main::solve;

    #[main]
    fn main() {
        let args = os::args();
        let mut iter = args.iter();
        iter.next();
        for arg in iter {
            match from_str::<u32>(arg.as_slice()) {
                Some(n) => {
                    spawn(proc() {
                        let mut w = MemWriter::new();
                        match solve(&mut w, n) {
                            Ok(()) => (),
                            Err(problems::InvalidProblemNumber) =>
                                { drop(writeln!(stderr(), "Invalid problem number: {}", n)); return },
                            Err(problems::ProblemIoError(e)) =>
                                { drop(writeln!(stderr(), "{}", e)); return }
                        }
                        match str::from_utf8(w.unwrap().as_slice()) {
                            Some(str) => println!("Problem {}: {}", n, str),
                            None => drop(writeln!(stderr(), "Output for problem {} was not a string", n))
                        }
                    });
                },
                _ => println!("Not a number."),
            }
        }
    }
}
