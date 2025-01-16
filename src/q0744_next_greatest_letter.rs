//You are given an array of characters letters that is sorted in non-decreasing
//order, and a character target. There are at least two different characters in
//letters.
//
// Return the smallest character in letters that is lexicographically greater
//than target. If such a character does not exist, return the first character in
//letters.
//
//
// Example 1:
//
//
//Input: letters = ["c","f","j"], target = "a"
//Output: "c"
//Explanation: The smallest character that is lexicographically greater than
//'a' in letters is 'c'.
//
//
// Example 2:
//
//
//Input: letters = ["c","f","j"], target = "c"
//Output: "f"
//Explanation: The smallest character that is lexicographically greater than
//'c' in letters is 'f'.
//
//
// Example 3:
//
//
//Input: letters = ["x","x","y","y"], target = "z"
//Output: "x"
//Explanation: There are no characters in letters that is lexicographically
//greater than 'z' so we return letters[0].
//
//
//
// Constraints:
//
//
// 2 <= letters.length <= 10⁴
// letters[i] is a lowercase English letter.
// letters is sorted in non-decreasing order.
// letters contains at least two different characters.
// target is a lowercase English letter.
//
//
// Related Topics Array Binary Search 👍 4563 👎 2208

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut left = 0;
        let mut right = letters.len() - 1;
        while left < right {
            let mid = left + ((right - left) >> 1);
            if letters[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if letters[left] <= target {
            letters[0]
        } else {
            letters[left]
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_greatest_letter() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'),
            'f'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'),
            'x'
        );
    }
}
