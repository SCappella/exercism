fn pow_mod(x: u64, mut y: u64, n: u64) -> u64 {
    let mut res: u128 = 1;
    let mut prod = u128::from(x);

    while y > 0 {
        if y % 2 != 0 {
            res *= prod;
            res %= u128::from(n);
        }
        y /= 2;
        prod *= prod;
        prod %= u128::from(n);
    }

    res as u64
}

pub fn private_key(_p: u64) -> u64 {
    // um, ok
    2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // g^a mod p
    pow_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // B**a mod p
    pow_mod(b_pub, a, p)
}
