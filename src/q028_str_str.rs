//Given two strings needle and haystack, return the index of the first
//occurrence of needle in haystack, or -1 if needle is not part of haystack.
//
//
// Example 1:
//
//
//Input: haystack = "sadbutsad", needle = "sad"
//Output: 0
//Explanation: "sad" occurs at index 0 and 6.
//The first occurrence is at index 0, so we return 0.
//
//
// Example 2:
//
//
//Input: haystack = "leetcode", needle = "leeto"
//Output: -1
//Explanation: "leeto" did not occur in "leetcode", so we return -1.
//
//
//
// Constraints:
//
//
// 1 <= haystack.length, needle.length <= 10⁴
// haystack and needle consist of only lowercase English characters.
//
//
// Related Topics Two Pointers String String Matching 👍 5799 👎 409

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }

        let haystack_chars = haystack.into_bytes();
        let needle_chars = needle.into_bytes();
        let mut next = vec![0; needle_chars.len()];
        Self::get_next(&mut next, &needle_chars);
        let mut j = 0;
        for i in 0..haystack_chars.len() {
            while j > 0 && haystack_chars[i] != needle_chars[j] {
                j = next[j - 1];
            }

            if haystack_chars[i] == needle_chars[j] {
                j += 1;
            }

            if j == needle_chars.len() {
                return (i + 1 - needle_chars.len()) as i32;
            }
        }

        return -1;
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
            0,
            Solution::str_str(String::from("sadbutsad"), String::from("sad"))
        );
        assert_eq!(
            -1,
            Solution::str_str(String::from("leetcode"), String::from("leeto"))
        );
        assert_eq!(
            4,
            Solution::str_str(String::from("mississippi"), String::from("issip"))
        );
    }
}
