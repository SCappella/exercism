pub fn factors(mut n: u64) -> Vec<u64> {
    let mut output = Vec::new();

    for i in 2.. {
        while n % i == 0 {
            output.push(i);
            n /= i;
        }

        if n == 1 {
            break;
        }
    }

    output
}
