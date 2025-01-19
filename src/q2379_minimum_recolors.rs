//You are given a 0-indexed string blocks of length n, where blocks[i] is
//either 'W' or 'B', representing the color of the iᵗʰ block. The characters 'W' and
//'B' denote the colors white and black, respectively.
//
// You are also given an integer k, which is the desired number of consecutive
//black blocks.
//
// In one operation, you can recolor a white block such that it becomes a black
//block.
//
// Return the minimum number of operations needed such that there is at least
//one occurrence of k consecutive black blocks.
//
//
// Example 1:
//
//
//Input: blocks = "WBBWWBBWBW", k = 7
//Output: 3
//Explanation:
//One way to achieve 7 consecutive black blocks is to recolor the 0th, 3rd, and
//4th blocks
//so that blocks = "BBBBBBBWBW".
//It can be shown that there is no way to achieve 7 consecutive black blocks in
//less than 3 operations.
//Therefore, we return 3.
//
//
// Example 2:
//
//
//Input: blocks = "WBWBBBW", k = 2
//Output: 0
//Explanation:
//No changes need to be made, since 2 consecutive black blocks already exist.
//Therefore, we return 0.
//
//
//
// Constraints:
//
//
// n == blocks.length
// 1 <= n <= 100
// blocks[i] is either 'W' or 'B'.
// 1 <= k <= n
//
//
// Related Topics String Sliding Window 👍 759 👎 21

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut ans = k;
        let mut cnt = 0;
        let k = k as usize;
        let chars = blocks.as_bytes();

        for (i, &char) in chars.iter().enumerate() {
            if char == b'B' {
                cnt += 1;
            }

            if i >= k && chars[i - k] == b'B' {
                cnt -= 1;
            }
            ans = ans.min((k - cnt) as i32);
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
        assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
        assert_eq!(Solution::minimum_recolors("BWWWBB".to_string(), 6), 3);
    }
}
