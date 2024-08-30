//You are given an array nums consisting of positive integers where all
//integers have the same number of digits.
//
// The digit difference between two integers is the count of different digits
//that are in the same position in the two integers.
//
// Return the sum of the digit differences between all pairs of integers in
//nums.
//
//
// Example 1:
//
//
// Input: nums = [13,23,12]
//
//
// Output: 4
//
// Explanation: We have the following: - The digit difference between 13 and 23
//is 1. - The digit difference between 13 and 12 is 1. - The digit difference
//between 23 and 12 is 2. So the total sum of digit differences between all pairs of
//integers is 1 + 1 + 2 = 4.
//
// Example 2:
//
//
// Input: nums = [10,10,10,10]
//
//
// Output: 0
//
// Explanation: All the integers in the array are the same. So the total sum of
//digit differences between all pairs of integers will be 0.
//
//
// Constraints:
//
//
// 2 <= nums.length <= 10⁵
// 1 <= nums[i] < 10⁹
// All integers in nums have the same number of digits.
//
//
// Related Topics Array Hash Table Math Counting 👍 169 👎 17

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let m = nums[0].to_string().len();
        let mut ans = m * n * (n - 1) / 2;
        let mut cnt = vec![[0; 10]; m];

        for num in nums {
            let mut num = num;
            let mut i = 0;
            while num > 0 {
                let d = (num % 10) as usize;
                ans -= cnt[i][d];
                cnt[i][d] += 1;
                i += 1;
                num /= 10;
            }
        }
        ans as i64
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q3153_sum_digit_differences::Solution;

    #[test]
    fn test() {
        assert_eq!(4, Solution::sum_digit_differences(vec![13, 23, 12]));
        assert_eq!(0, Solution::sum_digit_differences(vec![10, 10, 10, 10]));
    }
}
