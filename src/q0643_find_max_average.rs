//You are given an integer array nums consisting of n elements, and an integer
//k.
//
// Find a contiguous subarray whose length is equal to k that has the maximum
//average value and return this value. Any answer with a calculation error less
//than 10⁻⁵ will be accepted.
//
//
// Example 1:
//
//
//Input: nums = [1,12,-5,-6,50,3], k = 4
//Output: 12.75000
//Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
//
//
// Example 2:
//
//
//Input: nums = [5], k = 1
//Output: 5.00000
//
//
//
// Constraints:
//
//
// n == nums.length
// 1 <= k <= n <= 10⁵
// -10⁴ <= nums[i] <= 10⁴
//
//
// Related Topics Array Sliding Window 👍 3700 👎 344
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = nums.iter().take(k as usize).sum::<i32>();
        let mut ans = sum;
        for i in k as usize..nums.len() {
            sum += nums[i] - nums[i - k as usize];
            ans = ans.max(sum);
        }
        ans as f64 / k as f64
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_max_average() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75000
        );
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.00000);
    }
}
