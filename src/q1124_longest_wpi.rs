//We are given hours, a list of the number of hours worked per day for a given
//employee.
//
// A day is considered to be a tiring day if and only if the number of hours
//worked is (strictly) greater than 8.
//
// A well-performing interval is an interval of days for which the number of
//tiring days is strictly larger than the number of non-tiring days.
//
// Return the length of the longest well-performing interval.
//
//
// Example 1:
//
//
//Input: hours = [9,9,6,0,6,6,9]
//Output: 3
//Explanation: The longest well-performing interval is [9,9,6].
//
//
// Example 2:
//
//
//Input: hours = [6,6,6]
//Output: 0
//
//
//
// Constraints:
//
//
// 1 <= hours.length <= 10⁴
// 0 <= hours[i] <= 16
//
//
// Related Topics Array Hash Table Stack Monotonic Stack Prefix Sum 👍 1461 👎 1
//19

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let n = hours.len();
        let mut s = vec![0; n + 1];
        let mut st = vec![0];
        let mut ans = 0;

        for (mut i, &x) in hours.iter().enumerate() {
            i += 1;
            s[i] = s[i - 1];
            s[i] += if x > 8 { 1 } else { -1 };
            if s[i] < s[st[st.len() - 1]] {
                st.push(i);
            }
        }

        for i in (0..n + 1).rev() {
            while !st.is_empty() && s[i] > s[st[st.len() - 1]] {
                ans = ans.max(i as i32 - st[st.len() - 1] as i32);
                st.pop();
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
    fn test_longest_wpi() {
        assert_eq!(Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
        assert_eq!(Solution::longest_wpi(vec![6, 6, 6]), 0);
    }
}
