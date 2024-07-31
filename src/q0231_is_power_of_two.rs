//Given an integer n, return true if it is a power of two. Otherwise, return
//false.
//
// An integer n is a power of two, if there exists an integer x such that n == 2
//ˣ.
//
//
// Example 1:
//
//
//Input: n = 1
//Output: true
//Explanation: 2⁰ = 1
//
//
// Example 2:
//
//
//Input: n = 16
//Output: true
//Explanation: 2⁴ = 16
//
//
// Example 3:
//
//
//Input: n = 3
//Output: false
//
//
//
// Constraints:
//
//
// -2³¹ <= n <= 2³¹ - 1
//
//
//
//Follow up: Could you solve it without loops/recursion?
//
// Related Topics Math Bit Manipulation Recursion 👍 6827 👎 433

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        return n & (n - 1) == 0;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0231_is_power_of_two::Solution;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_power_of_two(1));
        assert_eq!(true, Solution::is_power_of_two(16));
        assert_eq!(false, Solution::is_power_of_two(3));
        assert_eq!(false, Solution::is_power_of_two(0));
    }
}
