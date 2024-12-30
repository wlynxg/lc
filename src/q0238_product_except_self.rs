//Given an integer array nums, return an array answer such that answer[i] is
//equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
//integer.
//
// You must write an algorithm that runs in O(n) time and without using the
//division operation.
//
//
// Example 1:
// Input: nums = [1,2,3,4]
//Output: [24,12,8,6]
//
// Example 2:
// Input: nums = [-1,1,0,-3,3]
//Output: [0,0,9,0,0]
//
//
// Constraints:
//
//
// 2 <= nums.length <= 10⁵
// -30 <= nums[i] <= 30
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit
//integer.
//
//
//
// Follow up: Can you solve the problem in O(1) extra space complexity? (The
//output array does not count as extra space for space complexity analysis.)
//
// Related Topics Array Prefix Sum 👍 23373 👎 1479

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut suf = vec![0; n];
        let mut pre = 1;
        suf[n - 1] = 1;

        for i in (0..n - 1).rev() {
            suf[i] = suf[i + 1] * nums[i + 1];
        }

        for (i, x) in nums.iter().enumerate() {
            suf[i] *= pre;
            pre *= x;
        }

        suf
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_except_self() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![-1, 1, 0, -3, 3])
        );
    }
}
