//Given a binary array nums, return the maximum number of consecutive 1's in
//the array.
//
//
// Example 1:
//
//
//Input: nums = [1,1,0,1,1,1]
//Output: 3
//Explanation: The first two digits or the last three digits are consecutive 1s.
// The maximum number of consecutive 1s is 3.
//
//
// Example 2:
//
//
//Input: nums = [1,0,1,1,0,1]
//Output: 2
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// nums[i] is either 0 or 1.
//
//
// Related Topics Array 👍 6014 👎 461
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut cnt = 0;

        for num in nums {
            if num == 1 {
                cnt += 1;
                if cnt > max {
                    max = cnt;
                }
            } else {
                cnt = 0;
            }
        }
        max
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
