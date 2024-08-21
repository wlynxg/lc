//An ugly number is a positive integer whose prime factors are limited to 2, 3,
//and 5.
//
// Given an integer n, return true if n is an ugly number.
//
//
// Example 1:
//
//
//Input: n = 6
//Output: true
//Explanation: 6 = 2 × 3
//
//
// Example 2:
//
//
//Input: n = 1
//Output: true
//Explanation: 1 has no prime factors, therefore all of its prime factors are
//limited to 2, 3, and 5.
//
//
// Example 3:
//
//
//Input: n = 14
//Output: false
//Explanation: 14 is not ugly since it includes the prime factor 7.
//
//
//
// Constraints:
//
//
// -2³¹ <= n <= 2³¹ - 1
//
//
// Related Topics Math 👍 3407 👎 1718
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        if n <= 0 {
            return false;
        }

        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0263_is_ugly::Solution;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_ugly(6));
        assert_eq!(true, Solution::is_ugly(1));
        assert_eq!(false, Solution::is_ugly(14));
    }
}
