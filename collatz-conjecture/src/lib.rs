pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut num: u128 = n as u128;
    let mut steps: u64 = 0;

    while num != 1 {
        if num.is_multiple_of(2) {
            num /= 2;
        } else {
            num = 3 * num + 1
        }
        steps += 1;
    }

    Some(steps)
}
