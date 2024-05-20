pub fn number(input: &str) -> Option<String> {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();

    if digits.len() == 11 && digits.starts_with('1') {
        Some(digits[1..].to_string())
    } else if digits.len() == 10 {
        Some(digits)
    } else {
        None
    }
}
