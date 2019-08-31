fn isqrt(x: usize) -> usize {
    // the highest power of 4 less than usize::max_value();
    // (even number of zeros)
    let mut scale: usize = ((!0) >> 1) ^ ((!0) >> 2);
    // ideally, we could make this the highest power of 4 less than or equal to x

    let mut root = 0;
    let mut remainder = x;

    while scale > 0 {
        // at each step, root is the actual guess times sqrt(scale)
        // scale = 2**(2k)
        // root = guess * 2**(k + 1)
        // remainder = x - guess**2
        // changing the guess to guess + 2**k reduces the remainder
        // by 2 * guess * 2**k + 2**(2k) = root + scale
        //
        // guess is divisible by 2**(k + 1), since we haven't added any lower bits than that
        // hence, root is divisible by 2**(2k + 2), so
        // root + scale = root | scale
        // and even root/2 + scale = root/2 | scale
        if remainder >= root | scale {
            remainder -= root | scale;
            // guess = guess + 2**k
            // so root = root + 2 * scale
            // but we also need to shift root by one bit
            // so we get root = root/2 + scale = (root >> 1) | scale
            root >>= 1;
            root |= scale;
        } else {
            root >>= 1;
        }
        // now root = guess * 2**((k - 1) + 1)
        // and if k = 0 (last iteration), root is guess

        scale >>= 2;
        // now scale = 2**(2(k - 1))
        // or zero if this is the last iteration
    }

    root
}

/// find smallest x such that floor(x/2) * ceil(x/2) >= size
fn rectangle_size(size: usize) -> (usize, usize) {
    // would actually be more efficient with ceil(sqrt(size));
    let lower = isqrt(size);
    let upper = lower + 1;
    if lower * lower >= size {
        (lower, lower)
    } else if lower * upper >= size {
        (lower, upper)
    } else {
        (upper, upper)
    }
}

pub fn encrypt(input: &str) -> String {
    let mut stripped: Vec<_> = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let (rows, columns) = rectangle_size(stripped.len());
    stripped.resize(rows * columns, ' ');

    (0..columns)
        .map(|i| {
            // move a reference into the closure below, rather than the actual Vec
            let stripped = &stripped;
            (0..rows)
                .map(move |j| stripped[columns * j + i])
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
