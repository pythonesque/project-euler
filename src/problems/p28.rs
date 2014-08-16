pub fn run() {
    // Let n = 2 * k + 1.
    // corner_0 = 1
    // corner_1 = (corner_0 + 8 * k)
    // ...
    // corner_k = (corner_(k-1) + 8 * k)
    //
    // diag_0 = 1
    // diag_1 = corner_0 * 4 + (2 * k) + (4 * k) + (6 * k) + (8 * k) = 4 * (corner_0 + 5 * k)
    // ...
    // diag_k = 4 * ((corner_k - 1) + 5 * k)
    static size: uint = 1001;
    static kmax: uint = (size + 1) / 2;
    let mut corner = 1;
    let mut sum = 1;
    for k in range(1, kmax) {
        sum += 4 * (corner + 5 * k);
        corner += 8 * k;
    }
    println!("{}", sum);
}
