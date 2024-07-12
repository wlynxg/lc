//You are given two non-empty linked lists representing two non-negative
//integers. The digits are stored in reverse order, and each of their nodes contains a
//single digit. Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the
//number 0 itself.
//
//
// Example 1:
//
//
//Input: l1 = [2,4,3], l2 = [5,6,4]
//Output: [7,0,8]
//Explanation: 342 + 465 = 807.
//
//
// Example 2:
//
//
//Input: l1 = [0], l2 = [0]
//Output: [0]
//
//
// Example 3:
//
//
//Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//Output: [8,9,9,9,0,0,0,1]
//
//
//
// Constraints:
//
//
// The number of nodes in each linked list is in the range [1, 100].
// 0 <= Node.val <= 9
// It is guaranteed that the list represents a number that does not have
//leading zeros.
//
//
// Related Topics Linked List Math Recursion 👍 31093 👎 6194

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = ListNode::new(0);
        let mut current = &mut head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        return head.next;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    fn create_linked_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let node = ListNode { val, next: head };
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn test_example1() {
        let l1 = create_linked_list(&[2, 4, 3]);
        let l2 = create_linked_list(&[5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(format_linked_list(&result), "[7, 0, 8]".to_string());
    }

    #[test]
    fn test_example2() {
        let l1 = create_linked_list(&[0]);
        let l2 = create_linked_list(&[0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(format_linked_list(&result), "[0]".to_string());
    }

    #[test]
    fn test_example3() {
        let l1 = create_linked_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_linked_list(&[9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(
            format_linked_list(&result),
            "[8, 9, 9, 9, 0, 0, 0, 1]".to_string()
        );
    }

    fn format_linked_list(head: &Option<Box<ListNode>>) -> String {
        let mut current = head;
        let mut result = "[".to_string();
        while let Some(node) = current {
            result.push_str(&node.val.to_string());
            current = &node.next;
            if current.is_some() {
                result.push_str(", ");
            }
        }
        result.push(']');
        result
    }
}
