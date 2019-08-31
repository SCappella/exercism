pub fn primes_up_to(upper_bound: usize) -> Vec<usize> {
    let mut is_possibly_prime = vec![true; upper_bound + 1];
    if let Some(el) = is_possibly_prime.get_mut(0) {
        *el = false;
    }
    if let Some(el) = is_possibly_prime.get_mut(1) {
        *el = false;
    }

    let mut primes = Vec::new();

    for p in 2..=upper_bound {
        if is_possibly_prime[p] {
            primes.push(p);
            for mul in p..=upper_bound / p {
                is_possibly_prime[p * mul] = false;
            }
        }
    }

    primes
}
