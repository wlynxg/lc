//Given the head of a singly linked list, return true if it is a palindrome or
//false otherwise.
//
//
// Example 1:
//
//
//Input: head = [1,2,2,1]
//Output: true
//
//
// Example 2:
//
//
//Input: head = [1,2]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [1, 10⁵].
// 0 <= Node.val <= 9
//
//
//
//Follow up: Could you do it in
//O(n) time and
//O(1) space?
//
// Related Topics Linked List Two Pointers Stack Recursion 👍 16516 👎 884

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

pub struct Solution;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut data = vec![];
        let mut head = head;

        while let Some(node) = head {
            data.push(node.val);
            head = node.next;
        }

        let length = data.len();
        for i in 0..length / 2 {
            if data[i] != data[length - i - 1] {
                return false;
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    // 辅助函数：从 Vec 创建链表
    fn create_linked_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        for &val in vec.iter() {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        dummy_head.next
    }

    #[test]
    fn test_example1() {
        let head = create_linked_list(vec![1, 2, 2, 1]);
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn test_example2() {
        let head = create_linked_list(vec![1, 2]);
        assert_eq!(Solution::is_palindrome(head), false);
    }

    #[test]
    fn test_single_node() {
        let head = create_linked_list(vec![1]);
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn test_empty_list() {
        let head = create_linked_list(vec![]);
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn test_long_palindrome() {
        let head = create_linked_list(vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1]);
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn test_long_non_palindrome() {
        let head = create_linked_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(Solution::is_palindrome(head), false);
    }
}
