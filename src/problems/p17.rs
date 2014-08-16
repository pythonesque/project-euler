use std::iter::{AdditiveIterator, range_inclusive};
use std::str::MaybeOwned;

fn number_to_word(number: u16) -> MaybeOwned<'static> {
    match number {
        0 => "".into_maybe_owned(), // This is just there to make life a bit easier for us, it's not correct
        1 => "one".into_maybe_owned(),
        2 => "two".into_maybe_owned(),
        3 => "three".into_maybe_owned(),
        4 => "four".into_maybe_owned(),
        5 => "five".into_maybe_owned(),
        6 => "six".into_maybe_owned(),
        7 => "seven".into_maybe_owned(),
        8 => "eight".into_maybe_owned(),
        9 => "nine".into_maybe_owned(),
        10 => "ten".into_maybe_owned(),
        11 => "eleven".into_maybe_owned(),
        12 => "twelve".into_maybe_owned(),
        13 => "thirteen".into_maybe_owned(),
        14 => "fourteen".into_maybe_owned(),
        15 => "fifteen".into_maybe_owned(),
        16 => "sixteen".into_maybe_owned(),
        17 => "seventeen".into_maybe_owned(),
        18 => "eighteen".into_maybe_owned(),
        19 => "nineteen".into_maybe_owned(),
        20..29 => "twenty".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        30..39 => "thirty".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        40..49 => "forty".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        50..59 => "fifty".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        60..69 => "sixty".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        70..79 => "seventy".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        80..89 => "eighty".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        90..99 => "ninety".into_string().append(number_to_word(number % 10).as_slice()).into_maybe_owned(),
        100..999 => {
            let hundreds = number_to_word(number / 100).into_string().append("hundred".as_slice());
            let tens = number % 100;
            if tens == 0 {
                hundreds
            } else {
                hundreds.append("and").append(number_to_word(tens).as_slice())
            }.into_maybe_owned()
        },
        1000 => "onethousand".into_maybe_owned(),
        _ => fail!("Unsupported number {}", number)
    }
}

euler_problem!(b"6a979d4a9cf85135408529edc8a133d0", w, {
    let number_count_sum = range_inclusive(1, 1000).map(number_to_word).map(|word| word.len()).sum();
    write!(w, "{}", number_count_sum)
})
