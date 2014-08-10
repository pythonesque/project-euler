use std::iter::AdditiveIterator;

fn add_digit(first: u8, second: u8, carry: u8) -> (u8, u8) {
    let sum = first + second + carry;
    let (remainder, carry) = (sum % 10, sum / 10);
    (remainder, carry)
}

fn double_digits(digits: &mut Vec<u8>) {
    let mut carry = 0;
    // We add digits backwards, but it shouldn't matter as long as we're consistent about it.
    for digit in digits.mut_iter() {
        let (remainder, new_carry) = add_digit(*digit, *digit, carry);
        *digit = remainder;
        carry = new_carry;
    }
    while carry != 0 {
        let (remainder, new_carry) = (carry % 10, carry / 10);
        digits.push(remainder);
        carry = new_carry;
    }
}

pub fn run() {
    static power: uint = 1000;
    let mut digits = vec!(1);
    for i in range(0, power) {
        double_digits(&mut digits);
    }
    let digit_sum = digits.move_iter().map(|n| n as u16).sum();
    println!("{}", digit_sum);
}
