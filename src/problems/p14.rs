use std::iter::iterate;

euler_problem!(b"5052c3765262bb2c6be537abd60b305e", w, {
    static max: u32 = 1_000_000;
    /* Brute force approach */
    let max_collatz = range(2, max)
        .max_by( |&num| {
            iterate( |n| if n & 1 == 0 { n / 2 } else { 3 * n + 1}, num) // Collatz
                .take_while( |&n| n != 1 )
                .count()
        }).unwrap();
    write!(w, "{}", max_collatz)
})
