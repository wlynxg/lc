//Given a non-negative integer x, return the square root of x rounded down to
//the nearest integer. The returned integer should be non-negative as well.
//
// You must not use any built-in exponent function or operator.
//
//
// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
//
//
//
// Example 1:
//
//
//Input: x = 4
//Output: 2
//Explanation: The square root of 4 is 2, so we return 2.
//
//
// Example 2:
//
//
//Input: x = 8
//Output: 2
//Explanation: The square root of 8 is 2.82842..., and since we round it down
//to the nearest integer, 2 is returned.
//
//
//
// Constraints:
//
//
// 0 <= x <= 2³¹ - 1
//
//
// Related Topics Math Binary Search 👍 8115 👎 4496

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }

        let mut left = 0;
        let mut right = x >> 1;
        let mut middle = 0;
        while left <= right {
            middle = (left + right) >> 1;
            let middle_product = middle as i64 * middle as i64;
            if middle_product > x as i64 {
                right = middle - 1;
            } else if middle_product < x as i64 {
                left = middle + 1;
            } else {
                return middle;
            }
        }

        if middle * middle < x {
            return middle;
        }
        return middle - 1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(10, Solution::my_sqrt(100));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
        assert_eq!(1, Solution::my_sqrt(1));
        assert_eq!(46339, Solution::my_sqrt(2147395599));
    }
}
