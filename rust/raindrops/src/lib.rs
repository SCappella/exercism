#[inline]
pub fn raindrops(n: u32) -> String {
    let mut string = String::with_capacity(15);
    if n % 3 == 0 {
        string += "Pling";
    }
    if n % 5 == 0 {
        string += "Plang";
    }
    if n % 7 == 0 {
        string += "Plong";
    }

    if string.is_empty() {
        string = format!("{}", n)
    }

    string
}
