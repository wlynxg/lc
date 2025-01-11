//Given a string s and an integer k, return the maximum number of vowel letters
//in any substring of s with length k.
//
// Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.
//
//
// Example 1:
//
//
//Input: s = "abciiidef", k = 3
//Output: 3
//Explanation: The substring "iii" contains 3 vowel letters.
//
//
// Example 2:
//
//
//Input: s = "aeiou", k = 2
//Output: 2
//Explanation: Any substring of length 2 contains 2 vowels.
//
//
// Example 3:
//
//
//Input: s = "leetcode", k = 3
//Output: 2
//Explanation: "lee", "eet" and "ode" contain 2 vowels.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s consists of lowercase English letters.
// 1 <= k <= s.length
//
//
// Related Topics String Sliding Window 👍 3602 👎 138

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut vowel = 0;
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            if vowels.contains(&chars[i]) {
                vowel += 1;
            }
            if i < k as usize - 1 {
                continue;
            }
            ans = ans.max(vowel);
            if vowels.contains(&chars[i + 1 - k as usize]) {
                vowel -= 1;
            }
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_vowels() {
        assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
        assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
    }
}
