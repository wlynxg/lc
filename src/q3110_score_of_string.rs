//You are given a string s. The score of a string is defined as the sum of the
//absolute difference between the ASCII values of adjacent characters.
//
// Return the score of s.
//
//
// Example 1:
//
//
// Input: s = "hello"
//
//
// Output: 13
//
// Explanation:
//
// The ASCII values of the characters in s are: 'h' = 104, 'e' = 101, 'l' = 108,
// 'o' = 111. So, the score of s would be |104 - 101| + |101 - 108| + |108 - 108|
//+ |108 - 111| = 3 + 7 + 0 + 3 = 13.
//
// Example 2:
//
//
// Input: s = "zaz"
//
//
// Output: 50
//
// Explanation:
//
// The ASCII values of the characters in s are: 'z' = 122, 'a' = 97. So, the
//score of s would be |122 - 97| + |97 - 122| = 25 + 25 = 50.
//
//
// Constraints:
//
//
// 2 <= s.length <= 100
// s consists only of lowercase English letters.
//
//
// Related Topics String 👍 580 👎 37

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut ans = 0;
        for i in 0..bytes.len() - 1 {
            ans += (bytes[i] as i32 - bytes[i + 1] as i32).abs();
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q3110_score_of_string::Solution;

    #[test]
    fn test() {
        assert_eq!(13, Solution::score_of_string("hello".to_string()));
        assert_eq!(50, Solution::score_of_string("zaz".to_string()));
    }
}
