//Given an integer array nums sorted in non-decreasing order, return an array
//of the squares of each number sorted in non-decreasing order.
//
//
// Example 1:
//
//
//Input: nums = [-4,-1,0,3,10]
//Output: [0,1,9,16,100]
//Explanation: After squaring, the array becomes [16,1,0,9,100].
//After sorting, it becomes [0,1,9,16,100].
//
//
// Example 2:
//
//
//Input: nums = [-7,-3,2,3,11]
//Output: [4,9,9,49,121]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -10⁴ <= nums[i] <= 10⁴
// nums is sorted in non-decreasing order.
//
//
//
//Follow up: Squaring each element and sorting the new array is very trivial,
//could you find an
//O(n) solution using a different approach?
//
// Related Topics Array Two Pointers Sorting 👍 9281 👎 238

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut left_product = nums[left] * nums[left];
        let mut right = nums.len() - 1;
        let mut right_product = nums[right] * nums[right];
        let mut result = vec![0; nums.len()];

        for i in (1..nums.len()).rev().step_by(1) {
            if left_product >= right_product {
                result[i] = left_product;
                left += 1;
                left_product = nums[left] * nums[left];
            } else {
                result[i] = right_product;
                right -= 1;
                right_product = nums[right] * nums[right]
            }
        }
        result[0] = if left_product <= right_product {
            left_product
        } else {
            right_product
        };
        return result;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(
        //     vec![0, 1, 9, 16, 100],
        //     Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        // );
        // assert_eq!(
        //     vec![4, 9, 9, 49, 121],
        //     Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        // );
        // assert_eq!(vec![1], Solution::sorted_squares(vec![1]));
        assert_eq!(
            vec![1, 4, 9, 25],
            Solution::sorted_squares(vec![-5, -3, -2, -1])
        );
    }
}
