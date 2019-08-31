use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // solving a + b + c = n for a and substituting into a^2 + b^2 = c^2 gives
    // a = n - b - c
    // c = n^2/(2(n - b)) - b
    // and so a = n - n^2/(2(n - b))
    // so we can iterate over the factors of n^2/2 (O(n), probably can be reduced to O(sqrt(n)))
    // If i is a factor, n - b = i, so b = n - i
    // taking the larger factor simply switches a and b

    if sum % 2 != 0 {
        return HashSet::new();
    }

    let mut output = HashSet::new();

    let product = sum.pow(2) / 2;
    for i in 1..product {
        if i * i > product {
            break;
        }

        if product % i == 0 {
            let b = sum - i; // b <= sqrt(n^2/2) < n, so this can't underflow
            let c = product / i - b; // i + product/i >= 2 * sqrt(product) = sqrt(2) * n >= n > n - i = b
            if let Some(a) = sum.checked_sub(product / i) {
                // a is the only thing that could be 0
                if a == 0 {
                    continue
                }
                let mut triple = [a, b, c];
                triple.sort_unstable();
                output.insert(triple);
            }
        }
    }

    output
}
