pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut last_char = '\0';
    let mut count = 0;

    for c in source.chars() {
        if c == last_char {
            count += 1
        } else {
            match count {
                1 => result.push(last_char),
                _ if count > 1 => result.push_str(&format!("{}{}", count, last_char)),
                _ => {}
            }
            last_char = c;
            count = 1;
        }
    }
    match count {
        1 => result.push(last_char),
        _ if count > 1 => result.push_str(&format!("{}{}", count, last_char)),
        _ => (),
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in source.chars() {
        if c.is_numeric() {
            count = count * 10 + c.to_digit(10).unwrap();
        } else {
            match count {
                0..=1 => result.push(c),
                _ => drop((0..count).map(|_| result.push(c)).count()),
            }
            count = 0;
        }
    }

    result
}
