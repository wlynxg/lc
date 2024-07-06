//Given two strings s and t, return true if t is an anagram of s, and false
//otherwise.
//
// An Anagram is a word or phrase formed by rearranging the letters of a
//different word or phrase, typically using all the original letters exactly once.
//
//
// Example 1:
// Input: s = "anagram", t = "nagaram"
//Output: true
//
// Example 2:
// Input: s = "rat", t = "car"
//Output: false
//
//
// Constraints:
//
//
// 1 <= s.length, t.length <= 5 * 10⁴
// s and t consist of lowercase English letters.
//
//
//
// Follow up: What if the inputs contain Unicode characters? How would you
//adapt your solution to such a case?
//
// Related Topics Hash Table String Sorting 👍 12082 👎 400

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = vec![0; 26];

        for i in s.bytes() {
            map[i as usize - 'a' as usize] += 1;
        }

        for i in t.bytes() {
            map[i as usize - 'a' as usize] -= 1;
        }

        return map.iter().filter(|x| **x != 0).count() == 0;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::is_anagram(String::from("anagram"), String::from("nagaram"))
        );
        assert_eq!(
            false,
            Solution::is_anagram(String::from("rat"), String::from("car"))
        );
    }
}
