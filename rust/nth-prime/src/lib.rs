pub fn nth(n: usize) -> usize {
    let mut primes = Vec::with_capacity(n);

    for k in 2.. {
        let mut is_prime = true;
        for &p in &primes {
            if k % p == 0 {
                is_prime = false;
                break;
            }
            if p * p > k {
                break;
            }
        }
        if is_prime {
            if primes.len() == n {
                return k;
            }
            primes.push(k);
        }
    }

    panic!("Too large")
}
