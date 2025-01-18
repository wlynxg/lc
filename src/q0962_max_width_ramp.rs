//A ramp in an integer array nums is a pair (i, j) for which i < j and nums[i] <
//= nums[j]. The width of such a ramp is j - i.
//
// Given an integer array nums, return the maximum width of a ramp in nums. If
//there is no ramp in nums, return 0.
//
//
// Example 1:
//
//
//Input: nums = [6,0,8,2,1,5]
//Output: 4
//Explanation: The maximum width ramp is achieved at (i, j) = (1, 5): nums[1] =
//0 and nums[5] = 5.
//
//
// Example 2:
//
//
//Input: nums = [9,8,1,0,1,9,4,0,4,1]
//Output: 7
//Explanation: The maximum width ramp is achieved at (i, j) = (2, 9): nums[2] =
//1 and nums[9] = 1.
//
//
//
// Constraints:
//
//
// 2 <= nums.length <= 5 * 10⁴
// 0 <= nums[i] <= 5 * 10⁴
//
//
// Related Topics Array Two Pointers Stack Monotonic Stack 👍 2645 👎 88

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut stack = vec![0];
        for i in 1..nums.len() {
            if nums[stack[stack.len() - 1]] > nums[i] {
                stack.push(i);
            }
        }

        let mut j = (nums.len() - 1) as i32;
        while !stack.is_empty() && j >= 0 {
            while j >= 0 && nums[j as usize] < nums[stack[stack.len() - 1]] {
                j -= 1;
            }
            if j >= 0 {
                if let Some(i) = stack.pop() {
                    ans = ans.max(j as usize - i);
                }
            }
        }
        ans as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_width_ramp() {
        assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
            7
        );
    }
}
