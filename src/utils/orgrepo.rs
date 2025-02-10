use regex::Regex;

/// Checks if the provided input string is in the "org/repo" or "user/repo" format.
/// It returns true if the string contains exactly one '/' with non-empty segments on both sides.
pub fn is_valid_orgrepo(input: &str) -> bool {
    let re = Regex::new(r"^[^/]+/[^/]+$").unwrap();
    re.is_match(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_orgrepo() {
        let valid_inputs = vec![
            "org/repo",
            "user/repository",
            "someOrg/someRepo",
        ];
        for input in valid_inputs {
            assert!(is_valid_orgrepo(input), "Expected '{}' to be valid", input);
        }
    }

    #[test]
    fn test_invalid_orgrepo() {
        let invalid_inputs = vec![
            "orgrepo",          // Missing '/'
            "org/",             // Missing repo part
            "/repo",            // Missing org/user part
            "org/repo/extra",   // Too many segments
            "org//repo",        // Empty segment between slashes
        ];
        for input in invalid_inputs {
            assert!(!is_valid_orgrepo(input), "Expected '{}' to be invalid", input);
        }
    }
}