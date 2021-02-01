use isbn_verifier::isbn_10_to_isbn_13;

#[test]
fn test_invalid_isbn_10() {
    assert_eq!(isbn_10_to_isbn_13("3-598-21508-9"), None);
}

#[test]
fn test_valid_isbn_10() {
    assert_eq!(
        isbn_10_to_isbn_13("3-598-21508-8"),
        Some("978-3-598-21508-7".to_string())
    );
}

#[test]
fn test_valid_isbn_10_check_digit_0() {
    assert_eq!(
        isbn_10_to_isbn_13("3-598-21507-X"),
        Some("978-3-598-21507-0".to_string())
    );
}
