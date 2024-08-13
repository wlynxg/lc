//An array is considered special if every pair of its adjacent elements
//contains two numbers with different parity.
//
//
// You are given an array of integers nums. Return true if nums is a special
//array, otherwise, return false.
//
//
// Example 1:
//
//
// Input: nums = [1]
//
//
// Output: true
//
// Explanation:
//
// There is only one element. So the answer is true.
//
// Example 2:
//
//
// Input: nums = [2,1,4]
//
//
// Output: true
//
// Explanation:
//
// There is only two pairs: (2,1) and (1,4), and both of them contain numbers
//with different parity. So the answer is true.
//
// Example 3:
//
//
// Input: nums = [4,3,1,6]
//
//
// Output: false
//
// Explanation:
//
// nums[1] and nums[2] are both odd. So the answer is false.
//
//
// Constraints:
//
//
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 100
//
//
// Related Topics Array 👍 87 👎 7

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i] % 2 == nums[i - 1] % 2 {
                return false;
            }
        }
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q3151_is_array_special::Solution;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_array_special(vec![1]));
        assert_eq!(true, Solution::is_array_special(vec![2, 1, 4]));
        assert_eq!(false, Solution::is_array_special(vec![4, 3, 1, 6]));
    }
}
