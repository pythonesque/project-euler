use std::io::{BufferedReader, File};

pub fn words(path: &Path) -> Vec<Vec<Ascii>> {
    let mut words = Vec::new();
    let file = File::open(path);
    let mut reader = BufferedReader::new(file);
    loop {
        static sep: u8 = b',';
        let word = reader.read_until(sep).unwrap();
        let last = word.len() - 1;
        if word[last] == sep {
            words.push(word.slice(1, last - 1).to_ascii().into_vec());
        } else {
            words.push(word.slice(1, last).to_ascii().into_vec());
            break;
        }
    }
    words
}
