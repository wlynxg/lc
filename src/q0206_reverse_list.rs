//Given the head of a singly linked list, reverse the list, and return the
//reversed list.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5]
//Output: [5,4,3,2,1]
//
//
// Example 2:
//
//
//Input: head = [1,2]
//Output: [2,1]
//
//
// Example 3:
//
//
//Input: head = []
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000
//
//
//
// Follow up: A linked list can be reversed either iteratively or recursively.
//Could you implement both?
//
// Related Topics Linked List Recursion 👍 21528 👎 436

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

pub struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut previous = None;

        while let Some(mut node) = current.take() {
            current = node.next;
            node.next = previous;
            previous = Some(node);
        }
        previous
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    fn create_linked_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut current = None;
        for &value in values.iter().rev() {
            let new_node = Box::new(ListNode {
                val: value,
                next: current,
            });
            current = Some(new_node);
        }
        current
    }

    #[test]
    fn test_reverse_list_empty() {
        let head = None;
        let reversed = Solution::reverse_list(head);
        assert_eq!(reversed, None);
    }

    #[test]
    fn test_reverse_list_single_node() {
        let head = create_linked_list(&[1]);
        let reversed = Solution::reverse_list(head);
        assert_eq!(reversed.as_ref().unwrap().val, 1);
        assert_eq!(reversed.as_ref().unwrap().next, None);
    }

    #[test]
    fn test_reverse_list_multiple_nodes() {
        let head = create_linked_list(&[1, 2, 3, 4, 5]);
        let reversed = Solution::reverse_list(head);
        let mut current = reversed.as_ref();
        assert_eq!(current.map(|node| node.val), Some(5));
        current = current.and_then(|node| node.next.as_ref());
        assert_eq!(current.map(|node| node.val), Some(4));
        current = current.and_then(|node| node.next.as_ref());
        assert_eq!(current.map(|node| node.val), Some(3));
        current = current.and_then(|node| node.next.as_ref());
        assert_eq!(current.map(|node| node.val), Some(2));
        current = current.and_then(|node| node.next.as_ref());
        assert_eq!(current.map(|node| node.val), Some(1));
        assert_eq!(current.and_then(|node| node.next.as_ref()), None);
    }
}
