//Given an array of distinct integers nums and a target integer target, return
//the number of possible combinations that add up to target.
//
// The test cases are generated so that the answer can fit in a 32-bit integer.
//
//
//
// Example 1:
//
//
//Input: nums = [1,2,3], target = 4
//Output: 7
//Explanation:
//The possible combination ways are:
//(1, 1, 1, 1)
//(1, 1, 2)
//(1, 2, 1)
//(1, 3)
//(2, 1, 1)
//(2, 2)
//(3, 1)
//Note that different sequences are counted as different combinations.
//
//
// Example 2:
//
//
//Input: nums = [9], target = 3
//Output: 0
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 200
// 1 <= nums[i] <= 1000
// All the elements of nums are unique.
// 1 <= target <= 1000
//
//
//
// Follow up: What if negative numbers are allowed in the given array? How does
//it change the problem? What limitation we need to add to the question to allow
//negative numbers?
//
// Related Topics Array Dynamic Programming 👍 7503 👎 674

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let t = target as usize;
        let mut f = vec![0; t + 1];
        f[0] = 1;
        for i in 1..=t {
            for &num in nums.iter() {
                let x = num as usize;
                if i >= x {
                    f[i] += f[i - x];
                }
            }
        }
        f[t]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combination_sum4() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3].to_vec(), 4), 7);
        assert_eq!(Solution::combination_sum4(vec![9].to_vec(), 3), 0);
    }
}
