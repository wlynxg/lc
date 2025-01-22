//You are a professional robber planning to rob houses along a street. Each
//house has a certain amount of money stashed. All houses at this place are arranged
//in a circle. That means the first house is the neighbor of the last one.
//Meanwhile, adjacent houses have a security system connected, and it will automatically
//contact the police if two adjacent houses were broken into on the same night.
//
// Given an integer array nums representing the amount of money of each house,
//return the maximum amount of money you can rob tonight without alerting the
//police.
//
//
// Example 1:
//
//
//Input: nums = [2,3,2]
//Output: 3
//Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money =
//2), because they are adjacent houses.
//
//
// Example 2:
//
//
//Input: nums = [1,2,3,1]
//Output: 4
//Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//Total amount you can rob = 1 + 3 = 4.
//
//
// Example 3:
//
//
//Input: nums = [1,2,3]
//Output: 3
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 1000
//
//
// Related Topics Array Dynamic Programming 👍 10202 👎 167

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    fn rob1(nums: &[i32], start: usize, end: usize) -> i32 {
        let (mut f0, mut f1) = (0, 0);
        for i in start..end {
            (f0, f1) = (f1, f1.max(f0 + nums[i]));
        }
        f1
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        Self::rob1(&nums, 1, nums.len()).max(nums[0] + Self::rob1(&nums, 2, nums.len() - 1))
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    }
}
