use num::bigint::BigUint;
use math::factorial;

euler_problem!(b"928f3957168ac592c4215dcd04e0b678", w, {
    // For an n x n grid, the question is how many ways you can place n objects into n+1 bins, where
    // some of the bins may be empty (the "bins" are between potential down-movements, and the
    // "objects" are the right-hand movements).  This is equivalent to ((n + n + 1 - 1) choose n), or
    // (2n)! / ((n!)(n)!).
    static grid_size: u32 = 20;

    let numerator: BigUint = factorial(2 * grid_size);
    let denominator_component: BigUint = factorial(grid_size);
    let ways = numerator / (denominator_component * denominator_component);

    write!(w, "{}", ways)
})
