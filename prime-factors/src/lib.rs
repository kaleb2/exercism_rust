pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    if n == 1 {
        return factors;
    }
    let mut num: u64 = n;
    let mut p: u64 = 2u64;

    while num > 1 {
        if num.is_multiple_of(p) {
            factors.push(p);
            num /= p;
        } else {
            p += 1;
        }
    }

    factors
}
