use io::words;
use std::collections::hashmap::HashSet;
use std::iter::{AdditiveIterator, Unfold};

euler_problem!(b"82aa4b0af34c2313a562076992e50aa3", w, {
    static path: &'static str = "data/words.txt";
    let words = words(&Path::new(path));
    let triangles: HashSet<u32> = Unfold::new(1u32, |i| {
        *i += 1;
        Some(*i * (*i - 1) / 2)
    }).take(words.len()).collect();
    let count = words.iter()
        .map( |word| word.iter().map( |ch| (ch.to_byte() - b'A' + 1) as u32).sum() )
        .filter( |sum| triangles.contains(sum) )
        .count();
    write!(w, "{}", count)
})
