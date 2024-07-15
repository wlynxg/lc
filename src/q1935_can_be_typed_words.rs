//There is a malfunctioning keyboard where some letter keys do not work. All
//other keys on the keyboard work properly.
//
// Given a string text of words separated by a single space (no leading or
//trailing spaces) and a string brokenLetters of all distinct letter keys that are
//broken, return the number of words in text you can fully type using this keyboard.
//
//
// Example 1:
//
//
//Input: text = "hello world", brokenLetters = "ad"
//Output: 1
//Explanation: We cannot type "world" because the 'd' key is broken.
//
//
// Example 2:
//
//
//Input: text = "leet code", brokenLetters = "lt"
//Output: 1
//Explanation: We cannot type "leet" because the 'l' and 't' keys are broken.
//
//
// Example 3:
//
//
//Input: text = "leet code", brokenLetters = "e"
//Output: 0
//Explanation: We cannot type either word because the 'e' key is broken.
//
//
//
// Constraints:
//
//
// 1 <= text.length <= 10⁴
// 0 <= brokenLetters.length <= 26
// text consists of words separated by a single space without any leading or
//trailing spaces.
// Each word only consists of lowercase English letters.
// brokenLetters consists of distinct lowercase English letters.
//
//
// Related Topics Hash Table String 👍 589 👎 29

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken = broken_letters
            .chars()
            .collect::<std::collections::HashSet<_>>();

        text.split_ascii_whitespace()
            .filter(|x| !x.chars().any(|y| broken.contains(&y)))
            .count() as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q1935_can_be_typed_words::Solution;

    #[test]
    fn test() {
        assert_eq!(
            1,
            Solution::can_be_typed_words(String::from("hello world"), String::from("ad"))
        );
        assert_eq!(
            1,
            Solution::can_be_typed_words(String::from("leet code"), String::from("lt"))
        );
        assert_eq!(
            0,
            Solution::can_be_typed_words(String::from("leet code"), String::from("e"))
        );
    }
}
