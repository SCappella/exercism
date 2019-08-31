const SCALE_NAMES: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];
const SCALE: u64 = 1_000;

const BASE: usize = 10;
const NUMBER_NAMES: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS_NAMES: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn encode_grouping(n: usize) -> String {
    assert!(0 < n && n < SCALE as usize);

    let mut output = Vec::new();

    if n % (BASE * BASE) == 0 {
        // multiple of 100 (nothing to add)
    } else if n % (BASE * BASE) < NUMBER_NAMES.len() {
        // 1-19
        output.push(String::from(NUMBER_NAMES[n % (BASE * BASE)]))
    } else if n % BASE != 0 {
        // 20-99 and doesn't end in 0
        output.push(String::from(TENS_NAMES[(n / BASE) % BASE]) + "-" + NUMBER_NAMES[n % BASE]);
    } else {
        // 20-99 and ends in 0
        output.push(String::from(TENS_NAMES[(n / BASE) % BASE]));
    }

    if n / (BASE * BASE) != 0 {
        output.push("hundred".into());
        output.push(String::from(NUMBER_NAMES[n / (BASE * BASE)]))
    }

    output.reverse();
    output.join(" ")
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".into();
    }

    let mut output = Vec::new();

    let mut scale = 0;
    while n > 0 {
        if n % SCALE != 0 {
            if scale > 0 {
                output.push(String::from(SCALE_NAMES[scale]));
            }
            output.push(encode_grouping((n % SCALE) as usize))
        }

        scale += 1;
        n /= SCALE;
    }

    output.reverse();
    output.join(" ")
}
