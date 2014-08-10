use num::bigint::BigUint;
use problems::factorial;

pub fn run() {
    // For an n x n grid, the question is how many ways you can place n objects into n+1 bins, where
    // some of the bins may be empty (the "bins" are between potential down-movements, and the
    // "objects" are the right-hand movements).  This is equivalent to ((n + n + 1 - 1) choose n), or
    // (2n)! / ((n!)(n)!).
    static grid_size: u8 = 20;

    let grid_size_big: BigUint = FromPrimitive::from_u8(grid_size).unwrap();

    let ways = factorial(&(grid_size_big + grid_size_big)) / (factorial(&grid_size_big) * factorial(&grid_size_big));
    println!("{}", ways);
}
