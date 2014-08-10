use std::iter::iterate;

pub fn run() {
    static max: u32 = 1_000_000;
    /* Brute force approach */
    let max_collatz = range(2, max)
        .max_by( |&num| {
            iterate( |n| if n & 1 == 0 { n / 2 } else { 3 * n + 1}, num) // Collatz
                .take_while( |&n| n != 1 )
                .count()
        });
    println!("{}", max_collatz);
}
