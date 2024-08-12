//Given a pattern and a string s, find if s follows the same pattern.
//
// Here follow means a full match, such that there is a bijection between a
//letter in pattern and a non-empty word in s.
//
//
// Example 1:
//
//
//Input: pattern = "abba", s = "dog cat cat dog"
//Output: true
//
//
// Example 2:
//
//
//Input: pattern = "abba", s = "dog cat cat fish"
//Output: false
//
//
// Example 3:
//
//
//Input: pattern = "aaaa", s = "dog cat cat dog"
//Output: false
//
//
//
// Constraints:
//
//
// 1 <= pattern.length <= 300
// pattern contains only lower-case English letters.
// 1 <= s.length <= 3000
// s contains only lowercase English letters and spaces ' '.
// s does not contain any leading or trailing spaces.
// All the words in s are separated by a single space.
//
//
// Related Topics Hash Table String 👍 7273 👎 1035

use std::ops::Index;

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();

        if pattern.len() != words.len() {
            return false;
        }

        pattern
            .chars()
            .enumerate()
            .map(|(i, c)| pattern.chars().position(|x| x == c).unwrap())
            .collect::<Vec<usize>>()
            == words
                .iter()
                .enumerate()
                .map(|(i, &w)| words.iter().position(|&x| x == w).unwrap())
                .collect::<Vec<usize>>()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0290_word_pattern::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string())
        );
        assert_eq!(
            false,
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string())
        );
        assert_eq!(
            false,
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string())
        );
    }
}
