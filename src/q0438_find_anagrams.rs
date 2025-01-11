//Given two strings s and p, return an array of all the start indices of p's
//anagrams in s. You may return the answer in any order.
//
//
// Example 1:
//
//
//Input: s = "cbaebabacd", p = "abc"
//Output: [0,6]
//Explanation:
//The substring with start index = 0 is "cba", which is an anagram of "abc".
//The substring with start index = 6 is "bac", which is an anagram of "abc".
//
//
// Example 2:
//
//
//Input: s = "abab", p = "ab"
//Output: [0,1,2]
//Explanation:
//The substring with start index = 0 is "ab", which is an anagram of "ab".
//The substring with start index = 1 is "ba", which is an anagram of "ab".
//The substring with start index = 2 is "ab", which is an anagram of "ab".
//
//
//
// Constraints:
//
//
// 1 <= s.length, p.length <= 3 * 10⁴
// s and p consist of lowercase English letters.
//
//
// Related Topics Hash Table String Sliding Window 👍 12576 👎 346

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut cnt_s = [0; 26];
        let mut cnt_p = [0; 26];
        for byte in p.bytes() {
            cnt_p[(byte - b'a') as usize] += 1;
        }
        let mut ans = Vec::new();

        let bytes = s.as_bytes();
        for (index, &b) in bytes.iter().enumerate() {
            cnt_s[(b - b'a') as usize] += 1;
            if index + 1 < p.len() {
                continue;
            }
            let left = index + 1 - p.len();

            if cnt_s == cnt_p {
                ans.push(left as i32);
            }
            cnt_s[(bytes[left] - b'a') as usize] -= 1;
        }

        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_anagrams() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_owned()),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_owned()),
            vec![0, 1, 2]
        );
    }
}
