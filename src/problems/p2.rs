use std::mem::swap;

pub fn run() {
    let (mut fib_i, mut fib_j) = (1, 2);
    let mut sum_even = 0;
    while fib_i <= 4_000_000u {
        if fib_i % 2 == 0 {
            sum_even += fib_i
        }
        swap(&mut fib_i, &mut fib_j);
        fib_j += fib_i
    }
    println!("{}", sum_even);
}
