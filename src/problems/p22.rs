use problems::quicksort;
use std::ascii::{AsciiCast, AsciiStr};
use std::iter::AdditiveIterator;
use std::io::{BufferedReader, File};

pub fn run() {
    static path: &'static str = "data/names.txt";
    let mut names = Vec::new();
    {
        let file = File::open(&Path::new(path));
        let mut reader = BufferedReader::new(file);
        loop {
            static sep: u8 = b',';
            let name = reader.read_until(sep).unwrap();
            let last = name.len() - 1;
            if name[last] == sep {
                names.push(name.slice(1, last - 1).to_ascii().as_str_ascii().into_string());
            } else {
                names.push(name.slice(1, last).to_ascii().as_str_ascii().into_string());
                break;
            }
        }
    }
    quicksort(names.as_mut_slice());
    let sum = names.move_iter()
        .enumerate()
        .map( |(i, name)|
            (i + 1) * name.into_bytes().iter()
                .map( |ch| (ch - b'A' + 1) as uint)
                .sum()
        ).sum();
    println!("{}", sum);
}
