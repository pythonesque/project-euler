pub fn run() {
    //  (1) c <= a + b (triangle inequality)
    //  (2) 2c <= a + b + c = 1000
    //  (3) c <= 500
    for c in range(0, 500u32) {
        for b in range(0, 1000 - c) {
            let a = 1000 - c - b;
            // Invariant: a + b + c = 1000.
            // Now just have to check if it's a Pythagorean triplet.
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                break;
            }
        }
    }
}
