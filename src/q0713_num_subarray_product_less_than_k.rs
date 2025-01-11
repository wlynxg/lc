//Given an array of integers nums and an integer k, return the number of
//contiguous subarrays where the product of all the elements in the subarray is strictly
//less than k.
//
//
// Example 1:
//
//
//Input: nums = [10,5,2,6], k = 100
//Output: 8
//Explanation: The 8 subarrays that have product less than 100 are:
//[10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
//Note that [10, 5, 2] is not included as the product of 100 is not strictly
//less than k.
//
//
// Example 2:
//
//
//Input: nums = [1,2,3], k = 0
//Output: 0
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 3 * 10⁴
// 1 <= nums[i] <= 1000
// 0 <= k <= 10⁶
//
//
// Related Topics Array Binary Search Sliding Window Prefix Sum 👍 6997 👎 222

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let mut left = 0;
        let mut s = 1;
        let mut ans = 0;
        for right in 0..nums.len() {
            s *= nums[right];
            while s >= k {
                s /= nums[left];
                left += 1;
            }
            ans += right - left + 1;
        }
        ans as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_subarray_product_less_than_k() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }
}
