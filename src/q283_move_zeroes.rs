//Given an integer array nums, move all 0's to the end of it while maintaining
//the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
//
//
// Example 1:
// Input: nums = [0,1,0,3,12]
//Output: [1,3,12,0,0]
//
// Example 2:
// Input: nums = [0]
//Output: [0]
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -2³¹ <= nums[i] <= 2³¹ - 1
//
//
//
//Follow up: Could you minimize the total number of operations done?
//
// Related Topics Array Two Pointers 👍 16611 👎 453

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }

        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[index] = nums[i];
                index += 1;
            }
        }

        for i in index..nums.len() {
            nums[i] = 0;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(expected, input);

        let mut input = vec![0];
        let expected = vec![0];
        Solution::move_zeroes(&mut input);
        assert_eq!(expected, input);

        let mut input = vec![1, 0];
        let expected = vec![1, 0];
        Solution::move_zeroes(&mut input);
        assert_eq!(expected, input);
    }
}
