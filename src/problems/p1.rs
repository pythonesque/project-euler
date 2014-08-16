use std::iter::AdditiveIterator;

euler_problem!(b"e1edf9d1967ca96767dcc2b2d6df69f4", w, {
    let res = range(0, 1000u)
        .filter( |i| i % 3 == 0 || i % 5 == 0)
        .sum();
    write!(w, "{}", res)
})
