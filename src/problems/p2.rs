use std::mem::swap;

euler_problem!(b"4194eb91842c8e7e6df099ca73c38f28", w, {
    let (mut fib_i, mut fib_j) = (1, 2);
    let mut sum_even = 0;
    while fib_i <= 4_000_000u {
        if fib_i % 2 == 0 {
            sum_even += fib_i
        }
        swap(&mut fib_i, &mut fib_j);
        fib_j += fib_i
    }
    write!(w, "{}", sum_even)
})
