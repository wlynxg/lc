//You are given an integer array cost where cost[i] is the cost of iᵗʰ step on
//a staircase. Once you pay the cost, you can either climb one or two steps.
//
// You can either start from the step with index 0, or the step with index 1.
//
// Return the minimum cost to reach the top of the floor.
//
//
// Example 1:
//
//
//Input: cost = [10,15,20]
//Output: 15
//Explanation: You will start at index 1.
//- Pay 15 and climb two steps to reach the top.
//The total cost is 15.
//
//
// Example 2:
//
//
//Input: cost = [1,100,1,1,1,100,1,1,100,1]
//Output: 6
//Explanation: You will start at index 0.
//- Pay 1 and climb two steps to reach index 2.
//- Pay 1 and climb two steps to reach index 4.
//- Pay 1 and climb two steps to reach index 6.
//- Pay 1 and climb one step to reach index 7.
//- Pay 1 and climb two steps to reach index 9.
//- Pay 1 and climb one step to reach the top.
//The total cost is 6.
//
//
//
// Constraints:
//
//
// 2 <= cost.length <= 1000
// 0 <= cost[i] <= 999
//
//
// Related Topics Array Dynamic Programming 👍 11722 👎 1812
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut f0, mut f1) = (0, 0);
        for i in 1..cost.len() {
            (f0, f1) = (f1, (f1 + cost[i]).min(f0 + cost[i - 1]));
        }
        f1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
