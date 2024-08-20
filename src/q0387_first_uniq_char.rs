//Given a string s, find the first non-repeating character in it and return its
//index. If it does not exist, return -1.
//
//
// Example 1:
// Input: s = "leetcode"
//Output: 0
//
// Example 2:
// Input: s = "loveleetcode"
//Output: 2
//
// Example 3:
// Input: s = "aabb"
//Output: -1
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s consists of only lowercase English letters.
//
//
// Related Topics Hash Table String Queue Counting 👍 8991 👎 298

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();

        for (i, byte) in s.bytes().enumerate() {
            map.entry(byte).or_insert((0, i)).0 += 1;
        }

        let mut ret = -1;
        for entry in map.values() {
            if entry.0 == 1 {
                ret = if ret != -1 {
                    std::cmp::min(entry.1 as i32, ret)
                } else {
                    entry.1 as i32
                }
            }
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0387_first_uniq_char::Solution;

    #[test]
    fn test() {
        assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()));
        assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
        assert_eq!(-1, Solution::first_uniq_char("aabb".to_string()));
    }
}
