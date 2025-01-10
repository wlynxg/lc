//Given n non-negative integers representing an elevation map where the width
//of each bar is 1, compute how much water it can trap after raining.
//
//
// Example 1:
//
//
//Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
//Output: 6
//Explanation: The above elevation map (black section) is represented by array [
//0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section)
//are being trapped.
//
//
// Example 2:
//
//
//Input: height = [4,2,0,3,2,5]
//Output: 9
//
//
//
// Constraints:
//
//
// n == height.length
// 1 <= n <= 2 * 10⁴
// 0 <= height[i] <= 10⁵
//
//
// Related Topics Array Two Pointers Dynamic Programming Stack Monotonic Stack ?
//? 33093 👎 561

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
// array storage prefix and suffix
// impl Solution {
//     pub fn trap(height: Vec<i32>) -> i32 {
//         let n = height.len();
//         let mut pre_max = vec![0; n];
//         pre_max[0] = height[0];
//         let mut mh = height[0];
//         for i in 1..n {
//             mh = mh.max(height[i]);
//             pre_max[i] = mh;
//         }
//
//         let mut suf_max = vec![0; n];
//         suf_max[n - 1] = height[n - 1];
//         mh = height[n - 1];
//         for i in (0..n - 1).rev() {
//             mh = mh.max(height[i]);
//             suf_max[i] = mh;
//         }
//
//         let mut ans = 0;
//         for i in 0..n {
//             ans += pre_max[i].min(suf_max[i]) - height[i];
//         }
//         ans
//     }
// }

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut pre_max = 0;
        let mut suf_max = 0;
        let mut ans = 0;
        let (mut left, mut right) = (0, n - 1);

        while left < right {
            pre_max = pre_max.max(height[left]);
            suf_max = suf_max.max(height[right]);
            if pre_max < suf_max {
                ans += pre_max - height[left];
                left += 1;
            } else {
                ans += suf_max - height[right];
                right -= 1;
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
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
