//Given an array nums of n integers where nums[i] is in the range [1, n],
//return an array of all the integers in the range [1, n] that do not appear in nums.
//
//
// Example 1:
// Input: nums = [4,3,2,7,8,2,3,1]
//Output: [5,6]
//
// Example 2:
// Input: nums = [1,1]
//Output: [2]
//
//
// Constraints:
//
//
// n == nums.length
// 1 <= n <= 10⁵
// 1 <= nums[i] <= n
//
//
//
// Follow up: Could you do it without extra space and in O(n) runtime? You may
//assume the returned list does not count as extra space.
//
// Related Topics Array Hash Table 👍 9438 👎 493
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len() as i32;
        let mut ret = vec![];
        for i in 0..n {
            let v = (nums[i as usize] - 1) % n;
            nums[v as usize] += n;
        }

        for i in 0..n {
            if nums[i as usize] <= n {
                ret.push(i + 1)
            }
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0448_find_disappeared_numbers::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
        assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    }
}
