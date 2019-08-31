fn bottles(n: usize, first: bool) -> String {
    let mut output = if n == 0 {
        if first { "N" } else { "n" }.to_owned() + "o more"
    } else {
        n.to_string()
    };
    output += " bottle";
    if n != 1 {
        output += "s"
    }

    output + " of beer"
}

fn on_the_wall(n: usize, first: bool) -> String {
    bottles(n, first) + " on the wall"
}

pub fn verse(n: usize) -> String {
    format!(
        "{}, {}.\n{}, {}.\n",
        on_the_wall(n, true),
        bottles(n, false),
        if n == 0 {
            "Go to the store and buy some more".to_owned()
        } else {
            format!(
                "Take {} down and pass it around",
                if n == 1 { "it" } else { "one" }
            )
        },
        if n == 0 {
            on_the_wall(99, false)
        } else {
            on_the_wall(n - 1, false)
        }
    )
}

pub fn sing(start: usize, end: usize) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
