//Given a string s, return the maximum number of occurrences of any substring
//under the following rules:
//
//
// The number of unique characters in the substring must be less than or equal
//to maxLetters.
// The substring size must be between minSize and maxSize inclusive.
//
//
//
// Example 1:
//
//
//Input: s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
//Output: 2
//Explanation: Substring "aab" has 2 occurrences in the original string.
//It satisfies the conditions, 2 unique letters and size 3 (between minSize and
//maxSize).
//
//
// Example 2:
//
//
//Input: s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
//Output: 2
//Explanation: Substring "aaa" occur 2 times in the string. It can overlap.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// 1 <= maxLetters <= 26
// 1 <= minSize <= maxSize <= min(26, s.length)
// s consists of only lowercase English letters.
//
//
// Related Topics Hash Table String Sliding Window 👍 1140 👎 412

pub struct Solution;
use std::collections::{HashMap, HashSet};
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, _max_size: i32) -> i32 {
        let min_size = min_size as usize;
        let max_letters = max_letters as usize;
        let chars = s.chars().collect::<Vec<char>>();
        let mut map = HashMap::new();

        for i in 0..=chars.len() - min_size {
            let substring = &chars[i..i + min_size];
            let unique_chars: HashSet<_> = substring.iter().collect();

            if unique_chars.len() <= max_letters {
                let count = map.entry(substring).or_insert(0);
                *count += 1;
            }
        }

        *map.values().max().unwrap_or(&0)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_freq() {
        assert_eq!(Solution::max_freq(String::from("aababcaab"), 2, 3, 4), 2);
        assert_eq!(Solution::max_freq(String::from("aaaa"), 1, 3, 3), 2);
    }
}
