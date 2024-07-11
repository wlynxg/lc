//Given a positive integer num, return true if num is a perfect square or false
//otherwise.
//
// A perfect square is an integer that is the square of an integer. In other
//words, it is the product of some integer with itself.
//
// You must not use any built-in library function, such as sqrt.
//
//
// Example 1:
//
//
//Input: num = 16
//Output: true
//Explanation: We return true because 4 * 4 = 16 and 4 is an integer.
//
//
// Example 2:
//
//
//Input: num = 14
//Output: false
//Explanation: We return false because 3.742 * 3.742 = 14 and 3.742 is not an
//integer.
//
//
//
// Constraints:
//
//
// 1 <= num <= 2³¹ - 1
//
//
// Related Topics Math Binary Search 👍 4217 👎 305

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num <= 1 {
            return true;
        }

        let mut left = 0;
        let mut right = num >> 1;
        let mut middle = 0;

        while left <= right {
            middle = (left + right) >> 1;
            let middle_product = middle as i64 * middle as i64;

            if middle_product > num as i64 {
                right = middle - 1;
            } else if middle_product < num as i64 {
                left = middle + 1;
            } else {
                return true;
            }
        }

        if middle * middle == num {
            return true;
        }

        return false;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_perfect_square(16));
        assert_eq!(false, Solution::is_perfect_square(14));
        assert_eq!(true, Solution::is_perfect_square(9));
    }
}
