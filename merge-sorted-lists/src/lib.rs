pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let Some(left) = list1 else {
        return list2;
    };

    let Some(right) = list2 else {
        return Some(left);
    };

    if left.val <= right.val {
        return Some(Box::new(ListNode {
            val: left.val,
            next: merge_two_lists(left.next, Some(right)),
        }));
    } else {
        return Some(Box::new(ListNode {
            val: right.val,
            next: merge_two_lists(Some(left), right.next),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_arrys_have_numbers() {
        let list1 = ListNode::from_arr(vec![1, 2, 4]);
        let list2 = ListNode::from_arr(vec![1, 3, 4]);
        let result = merge_two_lists(list1, list2);
        let expected = ListNode::from_arr(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(result, expected);
    }

    #[test]
    fn neither_array_has_numbers() {
        let list1 = ListNode::from_arr(vec![]);
        let list2 = ListNode::from_arr(vec![]);
        let result = merge_two_lists(list1, list2);
        let expected = ListNode::from_arr(vec![]);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_array_has_numbers() {
        let list1 = ListNode::from_arr(vec![]);
        let list2 = ListNode::from_arr(vec![0]);
        let result = merge_two_lists(list1, list2);
        let expected = ListNode::from_arr(vec![0]);
        assert_eq!(result, expected);
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_arr(vals: Vec<i32>) -> Option<Box<ListNode>> {
        if vals.is_empty() {
            return None;
        }

        let mut iter = vals.iter().rev();
        let mut current_node = ListNode::new(*iter.next().unwrap());

        while let Some(val) = iter.next() {
            current_node = ListNode {
                val: *val,
                next: Some(Box::new(current_node)),
            }
        }

        Some(Box::new(current_node))
    }
}
