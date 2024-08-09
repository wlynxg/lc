//Given an integer num, repeatedly add all its digits until the result has only
//one digit, and return it.
//
//
// Example 1:
//
//
//Input: num = 38
//Output: 2
//Explanation: The process is
//38 --> 3 + 8 --> 11
//11 --> 1 + 1 --> 2
//Since 2 has only one digit, return it.
//
//
// Example 2:
//
//
//Input: num = 0
//Output: 0
//
//
//
// Constraints:
//
//
// 0 <= num <= 2³¹ - 1
//
//
//
// Follow up: Could you do it without any loop/recursion in O(1) runtime?
//
// Related Topics Math Simulation Number Theory 👍 4845 👎 1937

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0258_add_digits::Solution;

    #[test]
    fn test() {
        assert_eq!(2, Solution::add_digits(38));
        assert_eq!(0, Solution::add_digits(0));
        assert_eq!(9, Solution::add_digits(9));
    }
}
