//You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists into one sorted list. The list should be made by
//splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
//
// Example 1:
//
//
//Input: list1 = [1,2,4], list2 = [1,3,4]
//Output: [1,1,2,3,4,4]
//
//
// Example 2:
//
//
//Input: list1 = [], list2 = []
//Output: []
//
//
// Example 3:
//
//
//Input: list1 = [], list2 = [0]
//Output: [0]
//
//
//
// Constraints:
//
//
// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.
//
//
// Related Topics Linked List Recursion 👍 21795 👎 2120

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
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
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut current = &mut head;
        let mut list1 = list1;
        let mut list2 = list2;

        while let (Some(n1), Some(n2)) = (list1.as_ref(), list2.as_ref()) {
            if n1.val < n2.val {
                current.next = list1;
                current = current.next.as_mut().unwrap();
                list1 = current.next.take();
            } else {
                current.next = list2;
                current = current.next.as_mut().unwrap();
                list2 = current.next.take();
            }
        }

        current.next = if list1.is_some() { list1 } else { list2 };
        head.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &value in values.iter().rev() {
            let mut node = ListNode::new(value);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn test_merge_two_lists_example1() {
        let list1 = create_list(&[1, 2, 4]);
        let list2 = create_list(&[1, 3, 4]);
        let expected = create_list(&[1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn test_merge_two_lists_example2() {
        let list1 = None;
        let list2 = None;
        let expected = None;
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn test_merge_two_lists_example3() {
        let list1 = None;
        let list2 = create_list(&[0]);
        let expected = create_list(&[0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn test_merge_two_lists_single_element() {
        let list1 = create_list(&[1]);
        let list2 = create_list(&[2]);
        let expected = create_list(&[1, 2]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn test_merge_two_lists_empty_lists() {
        let list1 = None;
        let list2 = None;
        let expected = None;
        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }
}
