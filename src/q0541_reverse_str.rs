//Given a string s and an integer k, reverse the first k characters for every 2
//k characters counting from the start of the string.
//
// If there are fewer than k characters left, reverse all of them. If there are
//less than 2k but greater than or equal to k characters, then reverse the first
//k characters and leave the other as original.
//
//
// Example 1:
// Input: s = "abcdefg", k = 2
//Output: "bacdfeg"
//
// Example 2:
// Input: s = "abcd", k = 2
//Output: "bacd"
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of only lowercase English letters.
// 1 <= k <= 10⁴
//
//
// Related Topics Two Pointers String 👍 1979 👎 3801

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s_arr = s.into_bytes();
        let length = s_arr.len() as i32;
        let mut index = 0;

        while index < length {
            let start = index as usize;
            let end = ((index + k - 1) as usize).min(s_arr.len() - 1);
            Self::reverse(&mut s_arr, start, end);
            index += 2 * k;
        }

        return String::from_utf8(s_arr).unwrap();
    }

    fn reverse(s_arr: &mut [u8], mut start: usize, mut end: usize) {
        while start < end {
            (s_arr[start], s_arr[end]) = (s_arr[end], s_arr[start]);
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
        assert_eq!(
            String::from("bacdfeg"),
            Solution::reverse_str(String::from("abcdefg"), 2)
        );
        assert_eq!(
            String::from("bacd"),
            Solution::reverse_str(String::from("abcd"), 2)
        );
    }
}
