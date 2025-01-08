pub fn roman_to_int(s: String) -> i32 {
    let values = s.chars().map(|c| match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0
    });
    let mut range = values.peekable();
    let mut acc = 0;

    while let Some(mut current) = range.next() {
        while let Some(next) = range.peek() {
            if next <= &current {
                break;
            }

            let reduced_value = next - current;
            range.next();
            current = reduced_value;
        }

        acc += current;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn iii_is_3() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    pub fn lviii_is_58() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    pub fn mcmxciv_is_1994() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
