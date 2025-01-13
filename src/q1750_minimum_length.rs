//Given a string s consisting only of characters 'a', 'b', and 'c'. You are
//asked to apply the following algorithm on the string any number of times:
//
//
// Pick a non-empty prefix from the string s where all the characters in the
//prefix are equal.
// Pick a non-empty suffix from the string s where all the characters in this
//suffix are equal.
// The prefix and the suffix should not intersect at any index.
// The characters from the prefix and suffix must be the same.
// Delete both the prefix and the suffix.
//
//
// Return the minimum length of s after performing the above operation any
//number of times (possibly zero times).
//
//
// Example 1:
//
//
//Input: s = "ca"
//Output: 2
//Explanation: You can't remove any characters, so the string stays as is.
//
//
// Example 2:
//
//
//Input: s = "cabaabac"
//Output: 0
//Explanation: An optimal sequence of operations is:
//- Take prefix = "c" and suffix = "c" and remove them, s = "abaaba".
//- Take prefix = "a" and suffix = "a" and remove them, s = "baab".
//- Take prefix = "b" and suffix = "b" and remove them, s = "aa".
//- Take prefix = "a" and suffix = "a" and remove them, s = "".
//
// Example 3:
//
//
//Input: s = "aabccabba"
//Output: 3
//Explanation: An optimal sequence of operations is:
//- Take prefix = "aa" and suffix = "a" and remove them, s = "bccabb".
//- Take prefix = "b" and suffix = "bb" and remove them, s = "cca".
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s only consists of characters 'a', 'b', and 'c'.
//
//
// Related Topics Two Pointers String 👍 1251 👎 107

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let chars = s.as_bytes();
        let mut left = 0;
        let mut right = chars.len() - 1;
        while left < right && chars[left] == chars[right] {
            let c = chars[left];
            left += 1;
            while left <= right && chars[left] == c {
                left += 1;
            }
            while left <= right && chars[right] == c {
                right -= 1;
            }
        }
        (right + 1 - left) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::minimum_length("ca".to_string()), 2);
        assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
        assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
    }
}
