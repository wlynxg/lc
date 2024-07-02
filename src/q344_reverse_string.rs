//Write a function that reverses a string. The input string is given as an
//array of characters s.
//
// You must do this by modifying the input array in-place with O(1) extra
//memory.
//
//
// Example 1:
// Input: s = ["h","e","l","l","o"]
//Output: ["o","l","l","e","h"]
//
// Example 2:
// Input: s = ["H","a","n","n","a","h"]
//Output: ["h","a","n","n","a","H"]
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s[i] is a printable ascii character.
//
//
// Related Topics Two Pointers String 👍 8617 👎 1173

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let last_index = s.len() - 1;
        for i in 0..s.len() >> 1 {
            (s[i], s[last_index - i]) = (s[last_index - i], s[i]);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let output = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string(&mut input);
        assert_eq!(output, input);

        let mut input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let output = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        Solution::reverse_string(&mut input);
        assert_eq!(output, input);
    }
}
