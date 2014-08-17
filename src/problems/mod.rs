#[cfg(test)]
pub static SHA_SIZE: uint = 32;

macro_rules! euler_problem(
    ($hash:expr, $writer:ident, $body: block) => (
        pub fn run<T: Writer>($writer: &mut T) -> ::std::io::IoResult<()> {
            $body
        }

        #[cfg(test)]
        mod test {
            #[test]
            pub fn test() {
                let mut process = match ::std::io::Command::new("md5sum").spawn() {
                    Ok(p) => p,
                    Err(e) => {
                        drop(write!(::std::io::stderr(), "Test ignored: I/O error {}", e));
                        return
                    }
                };
                match process.stdin.take() {
                    Some(mut stdin) => match super::run(&mut stdin) {
                        Ok(p) => p,
                        Err(e) => {
                            drop(write!(::std::io::stderr(), "Test ignored: I/O error {}", e));
                            return
                        }
                    },
                    None => {
                        drop(write!(::std::io::stderr(), "Test ignored: No standard input."));
                        return
                    }
                }
                let mut hash = [0u8, .. ::problems::SHA_SIZE];
                match process.stdout {
                    Some(ref mut stdout) => match stdout.read_at_least(::problems::SHA_SIZE, hash) {
                        Ok(_) => (),
                        Err(e) => {
                            drop(write!(::std::io::stderr(), "Test ignored: I/O error {}", e));
                            return
                        }
                    },
                    None => {
                        drop(write!(::std::io::stderr(), "Test ignored: No standard output."));
                        return
                    }
                }
                let status = match process.wait() {
                    Ok(s) => s,
                    Err(e) => {
                        drop(write!(::std::io::stderr(), "Test ignored: I/O error {}", e));
                        return
                    }
                };
                assert!(status.success());
                assert!(hash.as_slice() == $hash);
            }
        }
    )
)

pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;
pub mod p10;
pub mod p11;
pub mod p12;
pub mod p13;
pub mod p14;
pub mod p15;
pub mod p16;
pub mod p17;
pub mod p18;
pub mod p19;
pub mod p20;
pub mod p21;
pub mod p22;
pub mod p23;
pub mod p24;
pub mod p25;
pub mod p26;
pub mod p27;
pub mod p28;
pub mod p29;
pub mod p30;
pub mod p32;
pub mod p34;
pub mod p35;
pub mod p36;
pub mod p37;
pub mod p38;
pub mod p41;
pub mod p43;
pub mod p48;
pub mod p67;

#[cfg(not(test))]
pub mod main {
    use std::io::IoError;

    pub enum ProblemErrorKind {
        InvalidProblemNumber,
        ProblemIoError(IoError),
    }

    pub fn solve<T: Writer>(w: &mut T, n: u32) -> Result<(), ProblemErrorKind> {
        match n {
            1 => super::p1::run(w),
            2 => super::p2::run(w),
            3 => super::p3::run(w),
            4 => super::p4::run(w),
            5 => super::p5::run(w),
            6 => super::p6::run(w),
            7 => super::p7::run(w),
            8 => super::p8::run(w),
            9 => super::p9::run(w),
            10 => super::p10::run(w),
            11 => super::p11::run(w),
            12 => super::p12::run(w),
            13 => super::p13::run(w),
            14 => super::p14::run(w),
            15 => super::p15::run(w),
            16 => super::p16::run(w),
            17 => super::p17::run(w),
            18 => super::p18::run(w),
            19 => super::p19::run(w),
            20 => super::p20::run(w),
            21 => super::p21::run(w),
            22 => super::p22::run(w),
            23 => super::p23::run(w),
            24 => super::p24::run(w),
            25 => super::p25::run(w),
            26 => super::p26::run(w),
            27 => super::p27::run(w),
            28 => super::p28::run(w),
            29 => super::p29::run(w),
            30 => super::p30::run(w),
            32 => super::p32::run(w),
            34 => super::p34::run(w),
            35 => super::p35::run(w),
            36 => super::p36::run(w),
            37 => super::p37::run(w),
            38 => super::p38::run(w),
            41 => super::p41::run(w),
            43 => super::p43::run(w),
            48 => super::p48::run(w),
            67 => super::p67::run(w),
            _ => return Err(InvalidProblemNumber)
        }.map_err( |err| ProblemIoError(err) )
    }
}
