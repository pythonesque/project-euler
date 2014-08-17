use math::Digits;

euler_problem!(b"f2a29ede8dc9fae7926dc7a4357ac25e", w, {
    static max: u8 = 9;
    /// precondition: max < base
    static base: u8 = 10;
    let mut digits = [0u8, ..max as uint];
    for (i, digit) in digits.mut_iter().rev().enumerate() {
        *digit = i as u8 + 1;
    }
    'a: loop {
        for m in range(1, max as uint) {
            // Since first term is multiplied by 1, this establishes a
            let mut iter = digits.iter();
            let a_digits: Vec<u8> = iter.by_ref().take(m).map( |&d| d).collect();
            let a: u64 = match Digits::from_digits(a_digits.iter().rev(), base) {
                Some(a) => a,
                None => break
            };
            let mut n = 1;
            loop {
                let b = (n + 1) * a;
                let b_digits: Vec<u8> = b.digits(base).collect();
                let mut iter2 = b_digits.iter().rev().peekable();
                if !iter2.by_ref().zip(iter.by_ref()).all( |(d1,d2)| *d1 == *d2 ) { break }
                if iter2.peek().is_some() { break }
                n += 1;
            }
            if n > 1 && iter.next().is_none() {
                break 'a;
            }
        }
        if !digits.prev_permutation() { break }
    }
    let max_n: u64 = Digits::from_digits(digits.iter().rev(), base).unwrap();
    write!(w, "{}", max_n)
})
