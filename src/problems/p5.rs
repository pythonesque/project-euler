euler_problem!(b"bc0d0a22a7a46212135ed0ba77d22f3a", w, {
    // This problem doesn't really require a computer...
    // 1, 2, 3, 2^2, 5, 2*3, 7, 2^3, 3^2, 2*5, 11, 2^2*3, 13, 2*7, 3*5, 2^4, 17, 2*3^2, 19, 2^2 * 5
    //    2  3  2    5       7  2    3         11         13            2    17         19
    write!(w, "{}", 2u * 3 * 2 * 5 * 7 * 2 * 3 * 11 * 13 * 2 * 17 * 19)
})
