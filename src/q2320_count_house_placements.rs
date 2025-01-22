//There is a street with n * 2 plots, where there are n plots on each side of
//the street. The plots on each side are numbered from 1 to n. On each plot, a
//house can be placed.
//
// Return the number of ways houses can be placed such that no two houses are
//adjacent to each other on the same side of the street. Since the answer may be
//very large, return it modulo 10⁹ + 7.
//
// Note that if a house is placed on the iᵗʰ plot on one side of the street, a
//house can also be placed on the iᵗʰ plot on the other side of the street.
//
//
// Example 1:
//
//
//Input: n = 1
//Output: 4
//Explanation:
//Possible arrangements:
//1. All plots are empty.
//2. A house is placed on one side of the street.
//3. A house is placed on the other side of the street.
//4. Two houses are placed, one on each side of the street.
//
//
// Example 2:
//
//
//Input: n = 2
//Output: 9
//Explanation: The 9 possible arrangements are shown in the diagram above.
//
//
//
// Constraints:
//
//
// 1 <= n <= 10⁴
//
//
// Related Topics Dynamic Programming 👍 601 👎 197
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let n = n as usize;
        let cnt = 1_000_000_007;
        let (mut f0, mut f1) = (1, 2);

        for _ in 2..=n {
            (f1, f0) = ((f1 + f0) % cnt, f1)
        }
        ((f1 as i64) * (f1 as i64) % cnt as i64) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_house_placements() {
        assert_eq!(Solution::count_house_placements(1), 4);
        assert_eq!(Solution::count_house_placements(2), 9);
        assert_eq!(Solution::count_house_placements(1000), 500478595);
    }
}
