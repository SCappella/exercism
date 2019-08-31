pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            b @ ')' | b @ ']' | b @ '}' => {
                if stack.pop() == Some(b) {
                    continue;
                } else {
                    return false;
                }
            }
            _ => continue,
        }
    }
    stack.is_empty()
}
