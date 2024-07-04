//Given a string s, check if it can be constructed by taking a substring of it
//and appending multiple copies of the substring together.
//
//
// Example 1:
//
//
//Input: s = "abab"
//Output: true
//Explanation: It is the substring "ab" twice.
//
//
// Example 2:
//
//
//Input: s = "aba"
//Output: false
//
//
// Example 3:
//
//
//Input: s = "abcabcabcabc"
//Output: true
//Explanation: It is the substring "abc" four times or the substring "abcabc"
//twice.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of lowercase English letters.
//
//
// Related Topics String String Matching 👍 6380 👎 520

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut next = vec![0; s.len()];
        let chars = s.into_bytes();
        let length = chars.len();
        Self::get_next(&mut next, &chars);

        if next[length - 1] != 0 && length % (length - (next[length - 1])) == 0 {
            return true;
        }
        return false;
    }

    fn get_next(next: &mut Vec<usize>, chars: &Vec<u8>) {
        let mut j = 0;
        next[0] = 0;

        for i in 1..chars.len() {
            while j > 0 && chars[i] != chars[j] {
                j = next[j - 1];
            }

            if chars[i] == chars[j] {
                j += 1;
            }
            next[i] = j;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::repeated_substring_pattern(String::from("abab"))
        );
        assert_eq!(
            false,
            Solution::repeated_substring_pattern(String::from("aba"))
        );
        assert_eq!(
            true,
            Solution::repeated_substring_pattern(String::from("abcabcabcabc"))
        );
    }
}
