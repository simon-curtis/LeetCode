pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let mut result = "".to_string();
    let mut i = 0;
    loop {
        let mut char: Option<char> = None;
        for str in &strs {
            match str.chars().nth(i) {
                Some(current) =>  match char {
                    Some(c) if c != current => return result,
                    None => char = Some(current),
                    _ => continue
                },
                None => return result
            }
        }

        result.push(char.unwrap());
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fl_prefix() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl");
    }
}
