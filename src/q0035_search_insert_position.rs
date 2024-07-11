//Given a sorted array of distinct integers and a target value, return the
//index if the target is found. If not, return the index where it would be if it were
//inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
// Example 1:
//
//
//Input: nums = [1,3,5,6], target = 5
//Output: 2
//
//
// Example 2:
//
//
//Input: nums = [1,3,5,6], target = 2
//Output: 1
//
//
// Example 3:
//
//
//Input: nums = [1,3,5,6], target = 7
//Output: 4
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -10⁴ <= nums[i] <= 10⁴
// nums contains distinct values sorted in ascending order.
// -10⁴ <= target <= 10⁴
//
//
// Related Topics Array Binary Search 👍 16058 👎 739

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = (nums.len() - 1) as i32;

        while left <= right {
            let middle = (left + right) >> 1;
            let middle_index = middle as usize;

            if target < nums[middle_index] {
                right = middle - 1;
            } else if target > nums[middle_index] {
                left = middle + 1;
            } else {
                return middle;
            }
        }

        return right + 1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }
}
