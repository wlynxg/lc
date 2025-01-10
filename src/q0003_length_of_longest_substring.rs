use std::collections::HashSet;

//Given a string s, find the length of the longest substring without repeating
//characters.
//
//
// Example 1:
//
//
//Input: s = "abcabcbb"
//Output: 3
//Explanation: The answer is "abc", with the length of 3.
//
//
// Example 2:
//
//
//Input: s = "bbbbb"
//Output: 1
//Explanation: The answer is "b", with the length of 1.
//
//
// Example 3:
//
//
//Input: s = "pwwkew"
//Output: 3
//Explanation: The answer is "wke", with the length of 3.
//Notice that the answer must be a substring, "pwke" is a subsequence and not a
//substring.
//
//
//
// Constraints:
//
//
// 0 <= s.length <= 5 * 10⁴
// s consists of English letters, digits, symbols and spaces.
//
//
// Related Topics Hash Table String Sliding Window 👍 40954 👎 1975
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut ans = 0;
        let mut window = HashSet::new();
        let bytes = s.into_bytes();

        for right in 0..bytes.len() {
            let c = bytes[right];

            while window.contains(&c) {
                window.remove(&bytes[left]);
                left += 1;
            }
            window.insert(c);
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
