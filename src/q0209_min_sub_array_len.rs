//Given an array of positive integers nums and a positive integer target,
//return the minimal length of a subarray whose sum is greater than or equal to target.
//If there is no such subarray, return 0 instead.
//
//
// Example 1:
//
//
//Input: target = 7, nums = [2,3,1,2,4,3]
//Output: 2
//Explanation: The subarray [4,3] has the minimal length under the problem
//constraint.
//
//
// Example 2:
//
//
//Input: target = 4, nums = [1,4,4]
//Output: 1
//
//
// Example 3:
//
//
//Input: target = 11, nums = [1,1,1,1,1,1,1,1]
//Output: 0
//
//
//
// Constraints:
//
//
// 1 <= target <= 10⁹
// 1 <= nums.length <= 10⁵
// 1 <= nums[i] <= 10⁴
//
//
//
//Follow up: If you have figured out the
//O(n) solution, try coding another solution of which the time complexity is
//O(n log(n)).
//
// Related Topics Array Binary Search Sliding Window Prefix Sum 👍 12543 👎 432

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result = i32::MAX;
        let mut sum = 0;
        let mut start = 0;

        for i in 0..nums.len() {
            sum += nums[i];

            while sum >= target {
                let length = i as i32 - start + 1;
                result = if result < length { result } else { length };
                sum -= nums[start as usize];
                start += 1;
            }
        }

        return if result == i32::MAX { 0 } else { result };
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
        assert_eq!(1, Solution::min_sub_array_len(4, vec![1, 4, 4]));
        assert_eq!(
            0,
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1])
        );
    }
}
