//Given an integer n, return true if it is a power of four. Otherwise, return
//false.
//
// An integer n is a power of four, if there exists an integer x such that n ==
//4ˣ.
//
//
// Example 1:
// Input: n = 16
//Output: true
//
// Example 2:
// Input: n = 5
//Output: false
//
// Example 3:
// Input: n = 1
//Output: true
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
// Related Topics Math Bit Manipulation Recursion 👍 3927 👎 398

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n & 0x2aaaaaaa == 0
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0342_is_power_of_four::Solution;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_power_of_four(16));
        assert_eq!(false, Solution::is_power_of_four(5));
        assert_eq!(true, Solution::is_power_of_four(1));
    }
}
