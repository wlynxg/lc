//Given the head of a sorted linked list, delete all duplicates such that each
//element appears only once. Return the linked list sorted as well.
//
//
// Example 1:
//
//
//Input: head = [1,1,2]
//Output: [1,2]
//
//
// Example 2:
//
//
//Input: head = [1,1,2,3,3]
//Output: [1,2,3]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 300].
// -100 <= Node.val <= 100
// The list is guaranteed to be sorted in ascending order.
//
//
// Related Topics Linked List 👍 8685 👎 306

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = head.as_mut();
        while let Some(node) = cur.take() {
            while let Some(front) = node.next.as_mut() {
                if node.val == front.val {
                    node.next = front.next.take();
                } else {
                    break;
                }
            }
            cur = node.next.as_mut();
        }
        head
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    fn create_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &num in nums.iter().rev() {
            let node = Box::new(ListNode {
                val: num,
                next: head,
            });
            head = Some(node);
        }
        head
    }

    #[test]
    fn test_delete_duplicates() {
        // Test case 1
        let head = create_linked_list(&[1, 1, 2]);
        let expected = create_linked_list(&[1, 2]);
        assert_eq!(Solution::delete_duplicates(head), expected);

        // Test case 2
        let head = create_linked_list(&[1, 1, 2, 3, 3]);
        let expected = create_linked_list(&[1, 2, 3]);
        assert_eq!(Solution::delete_duplicates(head), expected);

        // Test case 3
        let head = create_linked_list(&[]);
        let expected = create_linked_list(&[]);
        assert_eq!(Solution::delete_duplicates(head), expected);
    }
}
