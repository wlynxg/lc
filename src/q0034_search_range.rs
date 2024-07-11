//Given an array of integers nums sorted in non-decreasing order, find the
//starting and ending position of a given target value.
//
// If target is not found in the array, return [-1, -1].
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
// Example 1:
// Input: nums = [5,7,7,8,8,10], target = 8
//Output: [3,4]
//
// Example 2:
// Input: nums = [5,7,7,8,8,10], target = 6
//Output: [-1,-1]
//
// Example 3:
// Input: nums = [], target = 0
//Output: [-1,-1]
//
//
// Constraints:
//
//
// 0 <= nums.length <= 10⁵
// -10⁹ <= nums[i] <= 10⁹
// nums is a non-decreasing array.
// -10⁹ <= target <= 10⁹
//
//
// Related Topics Array Binary Search 👍 20402 👎 516

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = Self::get_left_border(&nums, target);
        let right = Self::get_right_border(&nums, target);

        if left == -2 || right == -2 {
            return vec![-1, -1];
        }

        if right - left > 1 {
            return vec![left + 1, right - 1];
        }
        return vec![-1, -1];
    }

    fn get_right_border(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = nums.len() as i32 - 1;
        let mut right_border = -2;

        while left <= right {
            let middle = (left + right) >> 1;
            let middle_index = middle as usize;

            if target < nums[middle_index] {
                right = middle - 1;
            } else {
                left = middle + 1;
                right_border = left;
            }
        }

        return right_border;
    }

    fn get_left_border(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = nums.len() as i32 - 1;
        let mut left_border = -2;

        while left <= right {
            let middle = (left + right) >> 1;
            let middle_index = middle as usize;

            if target <= nums[middle_index] {
                right = middle - 1;
                left_border = right;
            } else {
                left = middle + 1;
            }
        }

        return left_border;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![2, 2], 3));
    }
}
