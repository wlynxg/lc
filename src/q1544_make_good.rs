//Given a string s of lower and upper case English letters.
//
// A good string is a string which doesn't have two adjacent characters s[i]
//and s[i + 1] where:
//
//
// 0 <= i <= s.length - 2
// s[i] is a lower-case letter and s[i + 1] is the same letter but in upper-
//case or vice-versa.
//
//
// To make the string good, you can choose two adjacent characters that make
//the string bad and remove them. You can keep doing this until the string becomes
//good.
//
// Return the string after making it good. The answer is guaranteed to be
//unique under the given constraints.
//
// Notice that an empty string is also good.
//
//
// Example 1:
//
//
//Input: s = "leEeetcode"
//Output: "leetcode"
//Explanation: In the first step, either you choose i = 1 or i = 2, both will
//result "leEeetcode" to be reduced to "leetcode".
//
//
// Example 2:
//
//
//Input: s = "abBAcC"
//Output: ""
//Explanation: We have many possible scenarios, and all lead to the same answer.
// For example:
//"abBAcC" --> "aAcC" --> "cC" --> ""
//"abBAcC" --> "abBA" --> "aA" --> ""
//
//
// Example 3:
//
//
//Input: s = "s"
//Output: "s"
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 100
// s contains only lower and upper case English letters.
//
//
// Related Topics String Stack 👍 3030 👎 171
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn make_good(s: String) -> String {
        s.chars()
            .fold(vec![], |mut acc, c| {
                if let Some(last) = acc.last() {
                    if (*last as u8).abs_diff(c as u8) == 32 {
                        acc.pop();
                        return acc;
                    }
                }
                acc.push(c);
                acc
            })
            .into_iter()
            .collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q1544_make_good::Solution;

    #[test]
    fn test() {
        assert_eq!(
            "leetcode".to_string(),
            Solution::make_good("leEeetcode".to_string())
        );
        assert_eq!("".to_string(), Solution::make_good("abBAcC".to_string()));
    }
}
