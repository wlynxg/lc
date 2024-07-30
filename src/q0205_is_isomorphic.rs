//Given two strings s and t, determine if they are isomorphic.
//
// Two strings s and t are isomorphic if the characters in s can be replaced to
//get t.
//
// All occurrences of a character must be replaced with another character while
//preserving the order of characters. No two characters may map to the same
//character, but a character may map to itself.
//
//
// Example 1:
// Input: s = "egg", t = "add"
//Output: true
//
// Example 2:
// Input: s = "foo", t = "bar"
//Output: false
//
// Example 3:
// Input: s = "paper", t = "title"
//Output: true
//
//
// Constraints:
//
//
// 1 <= s.length <= 5 * 10⁴
// t.length == s.length
// s and t consist of any valid ascii character.
//
//
// Related Topics Hash Table String 👍 9030 👎 2098
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut hashs = std::collections::HashMap::with_capacity(26);
        let mut hasht = std::collections::HashMap::with_capacity(26);

        for (sc, tc) in s.chars().zip(t.chars()) {
            if *hashs.entry(sc).or_insert(tc) != tc || *hasht.entry(tc).or_insert(sc) != sc {
                return false;
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0205_is_isomorphic::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::is_isomorphic("egg".to_string(), "add".to_string())
        );
        assert_eq!(
            false,
            Solution::is_isomorphic("foo".to_string(), "bar".to_string())
        );
        assert_eq!(
            true,
            Solution::is_isomorphic("paper".to_string(), "title".to_string())
        );
        assert_eq!(
            false,
            Solution::is_isomorphic("badc".to_string(), "baba".to_string())
        );
    }
}
