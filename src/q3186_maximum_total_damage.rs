//A magician has various spells.
//
// You are given an array power, where each element represents the damage of a
//spell. Multiple spells can have the same damage value.
//
// It is a known fact that if a magician decides to cast a spell with a damage
//of power[i], they cannot cast any spell with a damage of power[i] - 2, power[i] -
// 1, power[i] + 1, or power[i] + 2.
//
// Each spell can be cast only once.
//
// Return the maximum possible total damage that a magician can cast.
//
//
// Example 1:
//
//
// Input: power = [1,1,3,4]
//
//
// Output: 6
//
// Explanation:
//
// The maximum possible damage of 6 is produced by casting spells 0, 1, 3 with
//damage 1, 1, 4.
//
// Example 2:
//
//
// Input: power = [7,1,6,6]
//
//
// Output: 13
//
// Explanation:
//
// The maximum possible damage of 13 is produced by casting spells 1, 2, 3 with
//damage 1, 6, 6.
//
//
// Constraints:
//
//
// 1 <= power.length <= 10⁵
// 1 <= power[i] <= 10⁹
//
//
// Related Topics Array Hash Table Two Pointers Binary Search Dynamic
//Programming Sorting Counting 👍 255 👎 30

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        // 统计每个数字的频率
        let mut cnt: HashMap<i32, i64> = HashMap::new();
        for &x in &power {
            *cnt.entry(x).or_insert(0) += 1;
        }

        // 提取所有唯一的数字并排序
        let mut a: Vec<i32> = cnt.keys().cloned().collect();
        a.sort();

        let n = a.len();
        let mut f = vec![0; n + 1]; // 动态规划数组
        let mut j = 0; // 滑动窗口的左边界

        // 动态规划递推
        for i in 0..n {
            let x = a[i];
            // 移动 j，确保 a[j] < x - 2
            while j < i && a[j] < x - 2 {
                j += 1;
            }
            // 选择当前数字 x 或不选择当前数字 x
            f[i + 1] = f[i].max(f[j] + x as i64 * cnt[&x]);
        }

        f[n] // 返回最大值
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
        assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 6]), 13);
        assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 3]), 10);
    }
}
