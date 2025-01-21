//You are a professional robber planning to rob houses along a street. Each
//house has a certain amount of money stashed, the only constraint stopping you from
//robbing each of them is that adjacent houses have security systems connected and
//it will automatically contact the police if two adjacent houses were broken
//into on the same night.
//
// Given an integer array nums representing the amount of money of each house,
//return the maximum amount of money you can rob tonight without alerting the
//police.
//
//
// Example 1:
//
//
//Input: nums = [1,2,3,1]
//Output: 4
//Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//Total amount you can rob = 1 + 3 = 4.
//
//
// Example 2:
//
//
//Input: nums = [2,7,9,3,1]
//Output: 12
//Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5
//(money = 1).
//Total amount you can rob = 2 + 9 + 1 = 12.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 400
//
//
// Related Topics Array Dynamic Programming 👍 21739 👎 455

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // pub fn rob(nums: Vec<i32>) -> i32 {
    //     let mut memo = vec![-1; nums.len()];
    //     fn dfs(i: i32, nums: &[i32], memo: &mut [i32]) -> i32 {
    //         if i < 0 {
    //             return 0;
    //         }
    //
    //         if memo[i as usize] != -1 {
    //             return memo[i as usize];
    //         }
    //
    //         let ret = dfs(i - 1, nums, memo).max(dfs(i - 2, nums, memo) + nums[i as usize]);
    //         memo[i as usize] = ret;
    //         ret
    //     }
    //     dfs(nums.len() as i32 - 1, &nums, &mut memo)
    // }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f0 = 0;
        let mut f1 = 0;

        for i in 0..n {
            (f1, f0) = (f1.max(f0 + nums[i]), f1);
        }

        f1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
