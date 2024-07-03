//Given an input string s, reverse the order of the words.
//
// A word is defined as a sequence of non-space characters. The words in s will
//be separated by at least one space.
//
// Return a string of the words in reverse order concatenated by a single space.
//
//
// Note that s may contain leading or trailing spaces or multiple spaces
//between two words. The returned string should only have a single space separating the
//words. Do not include any extra spaces.
//
//
// Example 1:
//
//
//Input: s = "the sky is blue"
//Output: "blue is sky the"
//
//
// Example 2:
//
//
//Input: s = "  hello world  "
//Output: "world hello"
//Explanation: Your reversed string should not contain leading or trailing
//spaces.
//
//
// Example 3:
//
//
//Input: s = "a good   example"
//Output: "example good a"
//Explanation: You need to reduce multiple spaces between two words to a single
//space in the reversed string.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s contains English letters (upper-case and lower-case), digits, and spaces '
//'.
// There is at least one word in s.
//
//
//
// Follow-up: If the string data type is mutable in your language, can you
//solve it in-place with O(1) extra space?
//
// Related Topics Two Pointers String 👍 8359 👎 5159
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut chars = s.into_bytes();
        let length = Self::remove_space(&mut chars);

        Self::reverse_array(&mut chars, 0, length - 1);

        let mut start = 0;
        for i in 0..length {
            if i >= 1 && chars[i] == ' ' as u8 {
                Self::reverse_array(&mut chars, start, i - 1);
                start = i + 1;
            }
        }
        Self::reverse_array(&mut chars, start, length - 1);
        chars.truncate(length);
        return String::from_utf8(chars).unwrap();
    }

    fn remove_space(chars: &mut Vec<u8>) -> usize {
        let mut slow = 0;
        let mut fast = 0;
        let length = chars.len();

        for i in 0..length {
            if chars[i] != ' ' as u8 {
                break;
            }
            fast += 1;
        }

        for i in fast..length {
            if i > 1 && chars[i - 1] == chars[i] && chars[i - 1] == ' ' as u8 {
                continue;
            }
            chars[slow] = chars[i];
            slow += 1;
        }
        if slow > 1 && chars[slow - 1] == ' ' as u8 {
            return slow - 1;
        }
        return slow;
    }

    fn reverse_array(chars: &mut Vec<u8>, mut start: usize, mut end: usize) {
        while start < end {
            chars.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        // assert_eq!(
        //     String::from("blue is sky the"),
        //     Solution::reverse_words(String::from("the sky is blue"))
        // );
        assert_eq!(
            String::from("world hello"),
            Solution::reverse_words(String::from("  hello world  "))
        );
        assert_eq!(
            String::from("example good a"),
            Solution::reverse_words(String::from("a good   example"))
        );
    }
}
