pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut current = -101;
    let mut pos = 0;
    for index in 0..nums.len() {
        let next = nums[index];
        if next != current {
            current = next;
            nums[pos] = next;
            pos += 1;
        }
    }

    pos as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 2);
        assert_eq!(nums[..2], vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
    }
}
