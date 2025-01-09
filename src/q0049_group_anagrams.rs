//Given an array of strings strs, group the anagrams together. You can return
//the answer in any order.
//
//
// Example 1:
//
//
// Input: strs = ["eat","tea","tan","ate","nat","bat"]
//
//
// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//
// Explanation:
//
//
// There is no string in strs that can be rearranged to form "bat".
// The strings "nat" and "tan" are anagrams as they can be rearranged to form
//each other.
// The strings "ate", "eat", and "tea" are anagrams as they can be rearranged
//to form each other.
//
//
// Example 2:
//
//
// Input: strs = [""]
//
//
// Output: [[""]]
//
// Example 3:
//
//
// Input: strs = ["a"]
//
//
// Output: [["a"]]
//
//
// Constraints:
//
//
// 1 <= strs.length <= 10⁴
// 0 <= strs[i].length <= 100
// strs[i] consists of lowercase English letters.
//
//
// Related Topics Array Hash Table String Sorting 👍 19909 👎 660
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map = std::collections::HashMap::new();

        for s in strs {
            let mut sorted = s.clone().into_bytes();
            sorted.sort_unstable();
            anagram_map.entry(sorted).or_insert(vec![]).push(s);
        }
        anagram_map.into_values().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0049_group_anagrams::Solution;

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            vec![
                vec!["eat".to_string(), "tea".to_string(), "ate".to_string(),],
                vec!["nat".to_string(), "tan".to_string()],
                vec!["bat".to_string()],
            ],
            Solution::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ])
        );
        assert_eq!(
            vec![vec!["".to_string()]],
            Solution::group_anagrams(vec!["".to_string(),])
        );
        assert_eq!(
            vec![vec!["a".to_string()]],
            Solution::group_anagrams(vec!["a".to_string(),])
        );
    }
}
