use math::sieve;

euler_problem!(b"8c32ab09ec0210af60d392e9b2009560", w, {
    let mut n = 10_001u16;
    for prime in sieve(1_000_000/* guess at upper bound */) {
        n -= 1;
        if n == 0 {
            try!(write!(w, "{}", prime))
            break
        }
    }
    Ok(())
})
