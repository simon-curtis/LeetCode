pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    let word = x.to_string();
    let mut chars = word.chars();

    while let Some(l) = chars.next() {
        match chars.next_back() {
            Some(r) if l != r  => return false,
            _ => continue
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn valid_palendrome() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    pub fn negative_number() {
        assert_eq!(is_palindrome(-121), false);
    }
}
