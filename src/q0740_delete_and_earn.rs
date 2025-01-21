//You are given an integer array nums. You want to maximize the number of
//points you get by performing the following operation any number of times:
//
//
// Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must
//delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
//
//
//
// Return the maximum number of points you can earn by applying the above
//operation some number of times.
//
//
// Example 1:
//
//
//Input: nums = [3,4,2]
//Output: 6
//Explanation: You can perform the following operations:
//- Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
//- Delete 2 to earn 2 points. nums = [].
//You earn a total of 6 points.
//
//
// Example 2:
//
//
//Input: nums = [2,2,3,3,3,4]
//Output: 9
//Explanation: You can perform the following operations:
//- Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
//
//- Delete a 3 again to earn 3 points. nums = [3].
//- Delete a 3 once more to earn 3 points. nums = [].
//You earn a total of 9 points.
//
//
// Constraints:
//
//
// 1 <= nums.length <= 2 * 10⁴
// 1 <= nums[i] <= 10⁴
//
//
// Related Topics Array Hash Table Dynamic Programming 👍 7673 👎 390

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut nums_map = HashMap::new();
        for num in nums {
            *nums_map.entry(num).or_insert(0) += 1;
        }

        let (mut f0, mut f1) = (0, 0);
        let max_num = *nums_map.keys().max().unwrap_or(&0);

        for i in 0..max_num + 1 {
            (f0, f1) = (f1, f1.max(f0 + nums_map.get(&i).unwrap_or(&0) * i));
        }
        f1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_delete_and_earn() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
