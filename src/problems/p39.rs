use std::collections::hashmap::HashSet;

euler_problem!(b"fa83a11a198d5a7f0bf77a1987bcd006", w, {
    static max: u32 = 1000;
    let mut max_solns = 0u32;
    let mut max_p = 0;
    for p in range(3, max) {
        let mut solns = HashSet::new();
        for c in range(1, p - 2u32) {
            for a in range(1, p - c) {
                let b = p - a - c;
                if a * a + b * b == c * c {
                    solns.insert(if a < b { (a, b) } else { (b, a) } );
                }
            }
        }
        let len = solns.len() as u32;
        if len > max_solns {
            max_solns = len;
            max_p = p;
        }
    }
    write!(w, "{}", max_p)
})
