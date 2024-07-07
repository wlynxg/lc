//Given the head of a linked list and an integer val, remove all the nodes of
//the linked list that has Node.val == val, and return the new head.
//
//
// Example 1:
//
//
//Input: head = [1,2,6,3,4,5,6], val = 6
//Output: [1,2,3,4,5]
//
//
// Example 2:
//
//
//Input: head = [], val = 1
//Output: []
//
//
// Example 3:
//
//
//Input: head = [7,7,7,7], val = 7
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 10⁴].
// 1 <= Node.val <= 50
// 0 <= val <= 50
//
//
// Related Topics Linked List Recursion 👍 8282 👎 239

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dump = Box::new(ListNode::new(0));
        dump.next = head;
        let mut cur = dump.as_mut();

        while let Some(next) = cur.next.take() {
            if next.val == val {
                cur.next = next.next;
            } else {
                cur.next = Some(next);
                cur = cur.next.as_mut().unwrap();
            }
        }
        return dump.next;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
