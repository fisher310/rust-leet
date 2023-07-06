/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars: Vec<char> = isbn.chars().filter(|c| c.is_digit(10)).collect();
    if chars.len() < 1 {
        return false;
    }

    let last = isbn.chars().last().unwrap();
    if (chars.len() == 10 && last != 'X') || (chars.len() == 9 && last == 'X') {
        let mut res = chars.iter().take(9).enumerate().fold(0, |sum, (idx, ch)| {
            sum + (10 - idx) as u32 * ch.to_digit(10).unwrap()
        });

        res = 11 - (res % 11);

        if res == 10 {
            return last == 'X';
        } else {
            if let Some(v) = last.to_digit(10) {
                return v == res;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::isbn_verifier::is_valid_isbn;
    #[test]
    fn test_valid() {
        assert!(is_valid_isbn("3-598-21508-8"));
    }
    #[test]
    fn test_invalid_check_digit() {
        assert!(!is_valid_isbn("3-598-21508-9"));
    }
    #[test]
    fn test_valid_check_digit_of_10() {
        assert!(is_valid_isbn("3-598-21507-X"));
    }
    #[test]
    fn test_invalid_character_as_check_digit() {
        assert!(!is_valid_isbn("3-598-21507-A"));
    }
    #[test]
    fn test_invalid_character_in_isbn() {
        assert!(!is_valid_isbn("3-598-P1581-X"));
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_invalid_isbn_with_invalid_X() {
        assert!(!is_valid_isbn("3-598-2X507-9"));
    }
    #[test]
    fn test_valid_isbn_without_dashes() {
        assert!(is_valid_isbn("3598215088"));
    }
    #[test]
    fn test_valid_isbn_without_dashes_and_X_as_check() {
        assert!(is_valid_isbn("359821507X"));
    }
    #[test]
    fn test_invalid_isbn_without_dashes_and_no_check_digit() {
        assert!(!is_valid_isbn("359821507"));
    }
    #[test]
    fn test_invalid_isbn_without_dashes_and_too_long() {
        assert!(!is_valid_isbn("3598215078X"));
    }
    #[test]
    fn too_short_isbn() {
        assert!(!is_valid_isbn("00"));
    }
    #[test]
    fn test_invalid_isbn_without_check_digit() {
        assert!(!is_valid_isbn("3-598-21507"));
    }
    #[test]
    fn test_valid_digits_invalid_length() {
        assert!(!is_valid_isbn("35982150881"));
    }
    #[test]
    fn test_special_characters() {
        assert!(!is_valid_isbn("!@#%!@"));
    }
    #[test]
    fn test_invalid_isbn_with_check_digit_X_instead_of_0() {
        assert!(!is_valid_isbn("3-598-21515-X"));
    }
    #[test]
    fn empty_isbn() {
        assert!(!is_valid_isbn(""));
    }
    #[test]
    fn input_is_9_characters() {
        assert!(!is_valid_isbn("134456729"));
    }
    #[test]
    fn invalid_characters_are_not_ignored() {
        assert!(!is_valid_isbn("3132P34035"));
    }
    #[test]
    fn too_long_but_contains_a_valid_isbn() {
        assert!(!is_valid_isbn("98245726788"));
    }
}
