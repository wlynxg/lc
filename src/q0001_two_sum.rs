//Given an array of integers nums and an integer target, return indices of the
//two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may
//not use the same element twice.
//
// You can return the answer in any order.
//
//
// Example 1:
//
//
//Input: nums = [2,7,11,15], target = 9
//Output: [0,1]
//Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
//
// Example 2:
//
//
//Input: nums = [3,2,4], target = 6
//Output: [1,2]
//
//
// Example 3:
//
//
//Input: nums = [3,3], target = 6
//Output: [0,1]
//
//
//
// Constraints:
//
//
// 2 <= nums.length <= 10⁴
// -10⁹ <= nums[i] <= 10⁹
// -10⁹ <= target <= 10⁹
// Only one valid answer exists.
//
//
//
//Follow-up: Can you come up with an algorithm that is less than
//O(n²)
// time complexity?
//
// Related Topics Array Hash Table 👍 56401 👎 1952

use std::collections::HashMap;

pub struct Solution {}
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            map.insert(*num, i);
        }
        vec![]
    }
}

//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
