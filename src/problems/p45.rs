use std::iter::Unfold;

euler_problem!(b"30dfe3e3b286add9d12e493ca7be63fc", w, {
    static triangle_start: u32 = 285;
    static pentagon_start: u32 = 165;
    static hexagon_start: u32 = 143;
    let mut triangles = Unfold::new(triangle_start, |i| {
        *i += 1;
        Some(*i * (*i - 1) / 2)
    });
    let mut pentagonals = Unfold::new(pentagon_start, |i| {
        *i += 1;
        Some((*i - 1) * (3 * (*i - 1) - 1) / 2)
    });
    let mut hexagonals = Unfold::new(hexagon_start, |i| {
        *i += 1;
        Some((*i - 1) * (2 * (*i - 1) - 1))
    });

    triangles.next();
    let mut p: u32 = pentagonals.next().unwrap();
    let mut h: u32 = hexagonals.next().unwrap();
    let mut t: u32;
    loop {
        t = match triangles.next() { Some(t) => t, None => break };
        while p < t {
            p = match pentagonals.next() { Some(p) => p, None => break };
            while h < p {
                h = match hexagonals.next() { Some(h) => h, None => break };
            }
        }
        if t == p && p == h { break }
    }
    write!(w, "{}", t)
})
