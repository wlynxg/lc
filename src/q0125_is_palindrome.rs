//A phrase is a palindrome if, after converting all uppercase letters into
//lowercase letters and removing all non-alphanumeric characters, it reads the same
//forward and backward. Alphanumeric characters include letters and numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.
//
//
// Example 1:
//
//
//Input: s = "A man, a plan, a canal: Panama"
//Output: true
//Explanation: "amanaplanacanalpanama" is a palindrome.
//
//
// Example 2:
//
//
//Input: s = "race a car"
//Output: false
//Explanation: "raceacar" is not a palindrome.
//
//
// Example 3:
//
//
//Input: s = " "
//Output: true
//Explanation: s is an empty string "" after removing non-alphanumeric
//characters.
//Since an empty string reads the same forward and backward, it is a palindrome.
//
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 2 * 10⁵
// s consists only of printable ASCII characters.
//
//
// Related Topics Two Pointers String 👍 9376 👎 8367

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }

        let mut left = 0;
        let mut right = s.len() - 1;
        let chars = s.into_bytes();

        while left < right {
            while !chars[left].is_ascii_alphanumeric() && left < right {
                left += 1;
            }

            while !chars[right].is_ascii_alphanumeric() && left < right {
                right -= 1;
            }

            if left >= right {
                break;
            }

            if !chars[left].eq_ignore_ascii_case(&chars[right]) {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0125_is_palindrome::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"))
        );
        assert_eq!(false, Solution::is_palindrome(String::from("race a car")));
        assert_eq!(true, Solution::is_palindrome(String::from(" ")));
        assert_eq!(false, Solution::is_palindrome(String::from("0P")));
    }
}
