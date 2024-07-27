//Write a function to find the longest common prefix string amongst an array of
//strings.
//
// If there is no common prefix, return an empty string "".
//
//
// Example 1:
//
//
//Input: strs = ["flower","flow","flight"]
//Output: "fl"
//
//
// Example 2:
//
//
//Input: strs = ["dog","racecar","car"]
//Output: ""
//Explanation: There is no common prefix among the input strings.
//
//
//
// Constraints:
//
//
// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters.
//
//
// Related Topics String Trie 👍 17631 👎 4559

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let s0 = &strs[0];

        for (i, &char) in s0.as_bytes().iter().enumerate() {
            for str in strs[1..].iter() {
                if i == str.len() || str.as_bytes()[i] != char {
                    return s0[..i].to_string();
                }
            }
        }
        s0.to_string()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0014_longest_common_prefix::Solution;

    #[test]
    fn test() {
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(
                vec!["flower", "flow", "flight"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
        assert_eq!(
            "".to_string(),
            Solution::longest_common_prefix(
                vec!["dog", "racecar", "car"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
