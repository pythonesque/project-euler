#![macro_escape]

pub fn quicksort<'a, T: Clone + Ord>(array: &'a mut [T]) {
    let n = array.len();
    if n < 2 { return }
    let p = {
        // choose pivot
        let pivot_index = (n - 1) / 2;
        // partition
        let pivot_value = array[pivot_index].clone();
        array.swap(pivot_index, n - 1);
        let mut store_index = 0;
        for i in range(0, n - 1) {
            if array[i] < pivot_value {
                array.swap(i, store_index);
                store_index += 1;
            }
        }
        array.swap(store_index, n - 1);
        store_index
    };
    if p > 0 { quicksort(array.mut_slice(0, p)) }
    if p + 1 < n { quicksort(array.mut_slice(p + 1, n)) }
}

macro_rules! move_iterator(
    ($x: expr, $y: expr) => (
        for (a, b) in $x.mut_iter().zip($y) {
            *a = b
        }
    )
)
