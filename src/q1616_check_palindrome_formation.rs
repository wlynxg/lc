//You are given two strings a and b of the same length. Choose an index and
//split both strings at the same index, splitting a into two strings: aprefix and
//asuffix where a = aprefix + asuffix, and splitting b into two strings: bprefix and
//bsuffix where b = bprefix + bsuffix. Check if aprefix + bsuffix or bprefix +
//asuffix forms a palindrome.
//
// When you split a string s into sprefix and ssuffix, either ssuffix or
//sprefix is allowed to be empty. For example, if s = "abc", then "" + "abc", "a" + "bc",
// "ab" + "c" , and "abc" + "" are valid splits.
//
// Return true if it is possible to form a palindrome string, otherwise return
//false.
//
// Notice that x + y denotes the concatenation of strings x and y.
//
//
// Example 1:
//
//
//Input: a = "x", b = "y"
//Output: true
//Explaination: If either a or b are palindromes the answer is true since you
//can split in the following way:
//aprefix = "", asuffix = "x"
//bprefix = "", bsuffix = "y"
//Then, aprefix + bsuffix = "" + "y" = "y", which is a palindrome.
//
//
// Example 2:
//
//
//Input: a = "xbdef", b = "xecab"
//Output: false
//
//
// Example 3:
//
//
//Input: a = "ulacfd", b = "jizalu"
//Output: true
//Explaination: Split them at index 3:
//aprefix = "ula", asuffix = "cfd"
//bprefix = "jiz", bsuffix = "alu"
//Then, aprefix + bsuffix = "ula" + "alu" = "ulaalu", which is a palindrome.
//
//
//
// Constraints:
//
//
// 1 <= a.length, b.length <= 10⁵
// a.length == b.length
// a and b consist of lowercase English letters
//
//
// Related Topics Two Pointers String 👍 746 👎 252

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        Self::can_form_palindrome(&a_chars, &b_chars)
            || Self::can_form_palindrome(&b_chars, &a_chars)
    }

    fn can_form_palindrome(a: &[char], b: &[char]) -> bool {
        let mut i = 0;
        let mut j = a.len() - 1;

        while i < j && a[i] == b[j] {
            i += 1;
            j -= 1;
        }

        Self::is_palindrome(&a[i..=j]) || Self::is_palindrome(&b[i..=j])
    }

    fn is_palindrome(slice: &[char]) -> bool {
        let len = slice.len();
        for k in 0..len / 2 {
            if slice[k] != slice[len - 1 - k] {
                return false;
            }
        }
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_palindrome_formation() {
        assert_eq!(
            true,
            Solution::check_palindrome_formation("x".to_string(), "y".to_string())
        );
        assert_eq!(
            false,
            Solution::check_palindrome_formation("xbdef".to_string(), "xecab".to_string())
        );
        assert_eq!(
            true,
            Solution::check_palindrome_formation("ulacfd".to_string(), "jizalu".to_string())
        );
    }
}
