use std::char;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // insterting alphanumeric char into vector
    let mut isbn_10: Vec<char> = isbn.chars().filter(|c| c.is_alphanumeric()).collect();

    // check correct lenght
    if isbn_10.len() != 10 {
        // if invalid lenght then ISBN-10 is invalid
        return false;
    }

    let mut sum = 0;
    isbn_10.reverse();
    // calculating sum
    for (i, c) in isbn_10.iter().enumerate() {
        match c {
            '0'..='9' => {
                sum += c.to_digit(10).expect("Invalid input: Non-digit found!") * (i as u32 + 1);
            }
            'X' | 'x' => {
                // if X is on last position (i==0) then add 10 to sum
                if i == 0 {
                    sum += 10;
                // if not in last position then ISBN-10 is invalid
                } else {
                    return false;
                }
            } // if the character is invalid then ISBN-10 is invalid
            _ => return false,
        }
    }
    // check if correct ISBN-10
    if sum % 11 == 0 {
        true
    } else {
        false
    }
}

/// Converts ISBN-10 to ISBN-13
pub fn isbn_10_to_isbn_13(isbn: &str) -> Option<String> {
    // chechk if valid ISBN-10
    if !is_valid_isbn(isbn) {
        // if it is invalid ISB-10 return None
        return None;
    }
    // prepend ISBN-10 with "978" digits
    let mut isbn_13 = String::from("978")
        + &isbn
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
    // remove ISBN-10 check digit
    isbn_13.pop();

    // calulating sum of all digits
    let mut sum = 0;
    for (i, c) in isbn_13.chars().enumerate() {
        if (i + 1) % 2 == 0 {
            sum += c.to_digit(10).expect("Invalid input: Non-digit found!") * 3;
        } else {
            sum += c.to_digit(10).expect("Invalid input: Non-digit found!");
        }
    }
    // calulating remainder - check digit
    let remainder = sum % 10;
    if remainder == 0 {
        isbn_13.push('0');
    } else {
        let r = char::from_digit(10 - remainder, 10).expect("Should be valid number");
        isbn_13.push(r);
    }
    // insertibg hyphens
    let hyphens_position = [3, 5, 9, 15];
    for (_i, val) in hyphens_position.iter().enumerate() {
        isbn_13.insert(*val, '-');
    }
    // returnig valid ISBN-13 string
    Some(isbn_13)
}
