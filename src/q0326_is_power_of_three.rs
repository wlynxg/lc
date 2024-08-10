//Given an integer n, return true if it is a power of three. Otherwise, return
//false.
//
// An integer n is a power of three, if there exists an integer x such that n ==
// 3ˣ.
//
//
// Example 1:
//
//
//Input: n = 27
//Output: true
//Explanation: 27 = 3³
//
//
// Example 2:
//
//
//Input: n = 0
//Output: false
//Explanation: There is no x where 3ˣ = 0.
//
//
// Example 3:
//
//
//Input: n = -1
//Output: false
//Explanation: There is no x where 3ˣ = (-1).
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
// Related Topics Math Recursion 👍 3099 👎 277
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut m = n;
        if n == 1 {
            return true;
        } else if n <= 2 {
            return false;
        }
        while m > 1 {
            //println!("{}", m);
            if m % 3 != 0 {
                return false;
            }
            m = m / 3;
        }
        true
    }
}

//leetcode submit region end(Prohibit modification and deletion)
