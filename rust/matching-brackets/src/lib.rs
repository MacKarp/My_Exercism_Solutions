use std::vec;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_found = vec![];

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => bracket_found.push(c),
            ']' => match bracket_found.pop() {
                Some('[') => (),
                _ => return false,
            },
            '}' => match bracket_found.pop() {
                Some('{') => (),
                _ => return false,
            },
            ')' => match bracket_found.pop() {
                Some('(') => (),
                _ => return false,
            },
            _ => (),
        }
    }
    bracket_found.is_empty()
}
