//You are given a license key represented as a string s that consists of only
//alphanumeric characters and dashes. The string is separated into n + 1 groups by
//n dashes. You are also given an integer k.
//
// We want to reformat the string s such that each group contains exactly k
//characters, except for the first group, which could be shorter than k but still
//must contain at least one character. Furthermore, there must be a dash inserted
//between two groups, and you should convert all lowercase letters to uppercase.
//
// Return the reformatted license key.
//
//
// Example 1:
//
//
//Input: s = "5F3Z-2e-9-w", k = 4
//Output: "5F3Z-2E9W"
//Explanation: The string s has been split into two parts, each part has 4
//characters.
//Note that the two extra dashes are not needed and can be removed.
//
//
// Example 2:
//
//
//Input: s = "2-5g-3-J", k = 2
//Output: "2-5G-3J"
//Explanation: The string s has been split into three parts, each part has 2
//characters except the first part as it could be shorter as mentioned above.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s consists of English letters, digits, and dashes '-'.
// 1 <= k <= 10⁴
//
//
// Related Topics String 👍 1109 👎 1418
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut ret = String::new();
        let mut cnt = 0;

        for char in s.chars().rev().filter(|&x| x != '-') {
            if cnt == k {
                ret.push('-');
                cnt = 0;
            }

            ret.push(char);
            cnt += 1;
        }

        ret.chars().rev().collect::<String>().to_uppercase()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0482_license_key_formatting::Solution;

    #[test]
    fn test() {
        assert_eq!(
            "5F3Z-2E9W".to_string(),
            Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
        );
        assert_eq!(
            "2-5G-3J".to_string(),
            Solution::license_key_formatting("2-5g-3-J".to_string(), 2)
        );
    }
}
