//Given a string s consisting of words and spaces, return the length of the
//last word in the string.
//
// A word is a maximal substring consisting of non-space characters only.
//
//
// Example 1:
//
//
//Input: s = "Hello World"
//Output: 5
//Explanation: The last word is "World" with length 5.
//
//
// Example 2:
//
//
//Input: s = "   fly me   to   the moon  "
//Output: 4
//Explanation: The last word is "moon" with length 4.
//
//
// Example 3:
//
//
//Input: s = "luffy is still joyboy"
//Output: 6
//Explanation: The last word is "joyboy" with length 6.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of only English letters and spaces ' '.
// There will be at least one word in s.
//
//
// Related Topics String 👍 5216 👎 281

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let chars = s.chars();
        let mut cnt = 0;

        for c in chars.rev() {
            if c == ' ' as char && cnt > 0 {
                return cnt;
            }

            if c.is_alphabetic() {
                cnt += 1;
            }
        }

        cnt
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0058_length_of_last_word::Solution;

    #[test]
    fn test() {
        assert_eq!(
            5,
            Solution::length_of_last_word(String::from("Hello World"))
        );
        assert_eq!(
            4,
            Solution::length_of_last_word(String::from("   fly me   to   the moon  "))
        );
        assert_eq!(
            6,
            Solution::length_of_last_word(String::from("luffy is still joyboy"))
        );
    }
}
