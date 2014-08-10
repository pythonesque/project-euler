use problems::sieve;

pub fn run() {
    let mut n = 10_001u16;
    for prime in sieve(1_000_000/* guess at upper bound */) {
        n -= 1;
        if n == 0 {
            println!("{}", prime);
            break
        }
    }
}
