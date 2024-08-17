//You are given two positive integers low and high.
//
// An integer x consisting of 2 * n digits is symmetric if the sum of the first
//n digits of x is equal to the sum of the last n digits of x. Numbers with an
//odd number of digits are never symmetric.
//
// Return the number of symmetric integers in the range [low, high].
//
//
// Example 1:
//
//
//Input: low = 1, high = 100
//Output: 9
//Explanation: There are 9 symmetric integers between 1 and 100: 11, 22, 33, 44,
// 55, 66, 77, 88, and 99.
//
//
// Example 2:
//
//
//Input: low = 1200, high = 1230
//Output: 4
//Explanation: There are 4 symmetric integers between 1200 and 1230: 1203, 1212,
// 1221, and 1230.
//
//
//
// Constraints:
//
//
// 1 <= low <= high <= 10⁴
//
//
// Related Topics Math Enumeration 👍 252 👎 11
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high)
            .filter(|&i| {
                let s = i.to_string();
                if s.len() % 2 != 0 {
                    return false;
                }
                let (left, right) = s.split_at(s.len() / 2);
                left.bytes().sum::<u8>() == right.bytes().sum::<u8>()
            })
            .count() as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q2843_count_symmetric_integers::Solution;

    #[test]
    fn test() {
        assert_eq!(9, Solution::count_symmetric_integers(1, 100));
        assert_eq!(4, Solution::count_symmetric_integers(1200, 1230));
    }
}
