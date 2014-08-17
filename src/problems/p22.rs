use io::words;
use sort::quicksort;
use std::iter::AdditiveIterator;

euler_problem!(b"f2c9c91cb025746f781fa4db8be3983f", w, {
    static path: &'static str = "data/names.txt";
    let mut names = words(&Path::new(path));
    quicksort(names.as_mut_slice());
    let sum = names.move_iter()
        .enumerate()
        .map( |(i, name)|
            (i + 1) * name.into_bytes().iter()
                .map( |ch| (ch - b'A' + 1) as uint)
                .sum()
        ).sum();
    write!(w, "{}", sum)
})
