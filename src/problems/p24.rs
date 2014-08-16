euler_problem!(b"7f155b45cb3f0a6e518d59ec348bff84", w, {
    static max: uint = 10;
    static permute_max: uint = 1_000_000;
    let mut digits = [0u8, .. max];
    for (i, digit) in digits.mut_iter().enumerate() {
        *digit = i as u8;
    }
    for _ in range(1, permute_max) {
        let mut min_digit = 0;
        let mut min = 0u8;
        let mut max_digit = 0;
        for digit in range(1, max).rev() {
            let val = digits[digit - 1];
            if val < digits[digit] {
                min = val;
                min_digit = digit;
                break
            }
        }
        if min_digit == 0 { break }
        for (digit, &val) in digits.slice_from(min_digit).iter().enumerate().rev() {
            if min < val {
                max_digit = min_digit + digit;
                break
            }
        }
        digits.swap(min_digit - 1, max_digit);
        for digit in range(0, (max - min_digit + 1) / 2) {
            digits.swap(min_digit + digit, max - 1 - digit);
        }
    }
    for digit in digits.iter() {
        try!(write!(w, "{}", digit))
    }
    Ok(())
})
