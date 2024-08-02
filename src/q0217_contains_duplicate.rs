//Given an integer array nums, return true if any value appears at least twice
//in the array, and return false if every element is distinct.
//
//
// Example 1:
// Input: nums = [1,2,3,1]
//Output: true
//
// Example 2:
// Input: nums = [1,2,3,4]
//Output: false
//
// Example 3:
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
//Output: true
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -10⁹ <= nums[i] <= 10⁹
//
//
// Related Topics Array Hash Table Sorting 👍 12038 👎 1300

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());
        nums.into_iter().any(|num| !set.insert(num))
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0217_contains_duplicate::Solution;

    #[test]
    fn test() {
        assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert_eq!(false, Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert_eq!(
            true,
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
        );
    }
}
