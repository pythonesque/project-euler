use std::collections::hashmap::HashMap;

pub fn reciprocal_cycle(n: u64) -> u16 {
    let mut num = 1;
    let mut digits = HashMap::new();
    let mut digit = 0;
    while num != 0 {
        while num < n {
            num *= 10;
            digit += 1;
        }
        let rem = num % n;
        let old_digit = digits.find_or_insert(rem, digit);
        if *old_digit != digit {
            return digit - *old_digit;
        }
        num = rem;
    }
    return 0;
}

pub fn run() {
    static max_denominator: u64 = 1000;
    let max = range(1, max_denominator)
        .max_by( |&n| reciprocal_cycle(n) )
        .unwrap();
    println!("{}", max);
}
