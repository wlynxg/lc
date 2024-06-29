//Given two strings s and t, return true if they are equal when both are typed
//into empty text editors. '#' means a backspace character.
//
// Note that after backspacing an empty text, the text will continue empty.
//
//
// Example 1:
//
//
//Input: s = "ab#c", t = "ad#c"
//Output: true
//Explanation: Both s and t become "ac".
//
//
// Example 2:
//
//
//Input: s = "ab##", t = "c#d#"
//Output: true
//Explanation: Both s and t become "".
//
//
// Example 3:
//
//
//Input: s = "a#c", t = "b"
//Output: false
//Explanation: s becomes "c" while t becomes "b".
//
//
//
// Constraints:
//
//
// 1 <= s.length, t.length <= 200
// s and t only contain lowercase letters and '#' characters.
//
//
//
// Follow up: Can you solve it in O(n) time and O(1) space?
//
// Related Topics Two Pointers String Stack Simulation 👍 7492 👎 353
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        return Self::backspace_remove(&s) == Self::backspace_remove(&t);
    }

    pub fn backspace_remove(s: &String) -> String {
        let mut result = String::new();
        for i in s.chars() {
            if i != '#' {
                result.push(i);
            } else if !result.is_empty() {
                result.pop();
            }
        }

        return result;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q844_backspace_compare::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::backspace_compare("ab#c".parse().unwrap(), "ad#c".parse().unwrap())
        );
        assert_eq!(
            true,
            Solution::backspace_compare("ab##".parse().unwrap(), "c#d#".parse().unwrap())
        );
        assert_eq!(
            false,
            Solution::backspace_compare("a#c".parse().unwrap(), "b".parse().unwrap())
        );
    }
}
