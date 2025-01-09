//Given an unsorted array of integers nums, return the length of the longest
//consecutive elements sequence.
//
// You must write an algorithm that runs in O(n) time.
//
//
// Example 1:
//
//
//Input: nums = [100,4,200,1,3,2]
//Output: 4
//Explanation: The longest consecutive elements sequence is [1, 2, 3, 4].
//Therefore its length is 4.
//
//
// Example 2:
//
//
//Input: nums = [0,3,7,2,5,8,4,6,0,1]
//Output: 9
//
//
//
// Constraints:
//
//
// 0 <= nums.length <= 10⁵
// -10⁹ <= nums[i] <= 10⁹
//
//
// Related Topics Array Hash Table Union Find 👍 20803 👎 1084
pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let nums_set = nums.into_iter().collect::<HashSet<_>>();
        for &num in &nums_set {
            if nums_set.contains(&(num - 1)) {
                continue;
            }
            let mut y = num + 1;
            while nums_set.contains(&y) {
                y += 1;
            }
            ans = ans.max(y - num);
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_consecutive() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
        assert_eq!(
            9,
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        );
    }
}
