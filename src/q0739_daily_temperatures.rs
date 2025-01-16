//Given an array of integers temperatures represents the daily temperatures,
//return an array answer such that answer[i] is the number of days you have to wait
//after the iᵗʰ day to get a warmer temperature. If there is no future day for
//which this is possible, keep answer[i] == 0 instead.
//
//
// Example 1:
// Input: temperatures = [73,74,75,71,69,72,76,73]
//Output: [1,1,4,2,1,1,0,0]
//
// Example 2:
// Input: temperatures = [30,40,50,60]
//Output: [1,1,1,0]
//
// Example 3:
// Input: temperatures = [30,60,90]
//Output: [1,1,0]
//
//
// Constraints:
//
//
// 1 <= temperatures.length <= 10⁵
// 30 <= temperatures[i] <= 100
//
//
// Related Topics Array Stack Monotonic Stack 👍 13563 👎 340

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![0; temperatures.len()];

        for (i, &t) in temperatures.iter().enumerate() {
            while stack.len() > 0 && t > temperatures[stack[stack.len() - 1]] {
                if let Some(j) = stack.pop() {
                    ans[j] = (i - j) as i32;
                }
            }
            stack.push(i);
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
