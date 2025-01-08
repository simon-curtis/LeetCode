pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            '[' | '(' | '{' => stack.push(c),
            ']' => match stack.pop() {
                None => return false,
                Some(c) if c != '[' => return false,
                _ => continue,
            },
            ')' => match stack.pop() {
                None => return false,
                Some(c) if c != '(' => return false,
                _ => continue,
            },
            '}' => match stack.pop() {
                None => return false,
                Some(c) if c != '{' => return false,
                _ => continue,
            },
            _ => return false,
        }
    }

    return stack.is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_parens() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn balanced_all_types() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn unbalanced_paren_bracket() {
        assert_eq!(is_valid("(]".to_string()), false);
    }

    #[test]
    fn balanced_brackets_in_parens() {
        assert_eq!(is_valid("([])".to_string()), true);
    }
}
