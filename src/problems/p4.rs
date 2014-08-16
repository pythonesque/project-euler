euler_problem!(b"d4cfc27d16ea72a96b83d9bdef6ce2ec", w, {
    // Assuming the *largest* palindrome will have at least 6 digits
    let min = 99_999f64.sqrt() as u32 + 1;
    let mut max = 0;
    // (Very) brute force
    for i in range(min, 999) {
        'a: for j in range(min, 999) {
            let k = i * j;
            // No point in going on if it's too small.
            if k < max { continue };
            // Palindromic?
            let mut digits = [0u8, ..6];
            // Palindrome, so okay to do indices backwards
            let mut index = 0;
            let mut temp = k;
            while temp > 0 {
                digits[index] = (temp % 10) as u8;
                temp /= 10;
                index += 1;
            }
            for i in range(0, 3) {
                if digits[i] != digits[5 - i] { continue 'a }
            }
            max = k;
        }
    }
    write!(w, "{}", max)
})
