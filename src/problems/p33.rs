use math::gcd;

#[inline]
pub fn simplify(p: u8, q: u8) -> (u8, u8) {
    let d = gcd(p, q);
    (p / d, q / d)
}

#[inline]
pub fn cancel_shared_digits(p: [u8, .. 2], q: [u8, .. 2]) -> Option<(u8, u8)> {
    match (p[0], p[1], q[0], q[1]) {
        (0, _, _, _) => None, // trivial
        (_, 0, _, _) => None, // trivial
        (_, _, 0, _) => None, // trivial
        (_, _, _, 0) => None, // trivial
        (p, a, q, b) if a == b => Some((p, q)),
        (a, p, q, b) if a == b => Some((p, q)),
        (a, p, b, q) if a == b => Some((p, q)),
        (p, a, b, q) if a == b => Some((p, q)),
        _ => None,
    }
}

euler_problem!(b"f899139df5e1059396431415e770c6dd", w, {
    static base: u8 = 10;
    let mut prod = (1u32, 1u32);
    for p in range(base, base * base - 1) { // two digits
        let pd = [p / base, p % base];
        for q in range(p + 1, base * base) { // less than 1 in value
            let qd = [q / base, q % base];
            let (p0, q0) = match cancel_shared_digits(pd, qd) {
                Some(pair) => pair,
                None => continue,
            };
            if simplify(p0, q0) == simplify(p, q) {
                prod = match prod {
                    (r0, r1) => (r0 * p as u32, r1 * q as u32)
                }
            }
        }
    }
    let denom = match prod {
        (num, denom) => {
            let d = gcd(num, denom);
            denom / d
        }
    };
    write!(w, "{}", denom)
})
