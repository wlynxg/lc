//Given an array of integers arr and two integers k and threshold, return the
//number of sub-arrays of size k and average greater than or equal to threshold.
//
//
// Example 1:
//
//
//Input: arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
//Output: 3
//Explanation: Sub-arrays [2,5,5],[5,5,5] and [5,5,8] have averages 4, 5 and 6
//respectively. All other sub-arrays of size 3 have averages less than 4 (the
//threshold).
//
//
// Example 2:
//
//
//Input: arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
//Output: 6
//Explanation: The first 6 sub-arrays of size 3 have averages greater than 5.
//Note that averages are not integers.
//
//
//
// Constraints:
//
//
// 1 <= arr.length <= 10⁵
// 1 <= arr[i] <= 10⁴
// 1 <= k <= arr.length
// 0 <= threshold <= 10⁴
//
//
// Related Topics Array Sliding Window 👍 1667 👎 106
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let threshold_sum = threshold * k;
        let k = k as usize;
        let mut sum = arr.iter().take(k).sum::<i32>();
        let mut ans = 0;

        if sum >= threshold_sum {
            ans += 1;
        }
        for i in k..arr.len() {
            sum += arr[i] - arr[i - k];
            if sum >= threshold_sum {
                ans += 1;
            }
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_of_subarrays() {
        // assert_eq!(
        //     Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4),
        //     3
        // );
        assert_eq!(
            Solution::num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
            6
        );
    }
}
