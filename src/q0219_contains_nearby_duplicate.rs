//Given an integer array nums and an integer k, return true if there are two
//distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <
//= k.
//
//
// Example 1:
//
//
//Input: nums = [1,2,3,1], k = 3
//Output: true
//
//
// Example 2:
//
//
//Input: nums = [1,0,1,1], k = 1
//Output: true
//
//
// Example 3:
//
//
//Input: nums = [1,2,3,1,2,3], k = 2
//Output: false
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -10⁹ <= nums[i] <= 10⁹
// 0 <= k <= 10⁵
//
//
// Related Topics Array Hash Table Sliding Window 👍 6148 👎 3098

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut set = std::collections::HashSet::new();

        for i in 0..nums.len() {
            if i as i32 - k > 0 {
                set.remove(&nums[i - k as usize - 1]);
            }

            if !set.insert(nums[i]) {
                return true;
            }
        }
        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0219_contains_nearby_duplicate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
        );
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1)
        );
        assert_eq!(
            false,
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2)
        );
    }
}
