//There are several cards arranged in a row, and each card has an associated
//number of points. The points are given in the integer array cardPoints.
//
// In one step, you can take one card from the beginning or from the end of the
//row. You have to take exactly k cards.
//
// Your score is the sum of the points of the cards you have taken.
//
// Given the integer array cardPoints and the integer k, return the maximum
//score you can obtain.
//
//
// Example 1:
//
//
//Input: cardPoints = [1,2,3,4,5,6,1], k = 3
//Output: 12
//Explanation: After the first step, your score will always be 1. However,
//choosing the rightmost card first will maximize your total score. The optimal
//strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 1
//2.
//
//
// Example 2:
//
//
//Input: cardPoints = [2,2,2], k = 2
//Output: 4
//Explanation: Regardless of which two cards you take, your score will always
//be 4.
//
//
// Example 3:
//
//
//Input: cardPoints = [9,7,7,9,7,7,9], k = 7
//Output: 55
//Explanation: You have to take all the cards. Your score is the sum of points
//of all cards.
//
//
//
// Constraints:
//
//
// 1 <= cardPoints.length <= 10⁵
// 1 <= cardPoints[i] <= 10⁴
// 1 <= k <= cardPoints.length
//
//
// Related Topics Array Sliding Window Prefix Sum 👍 6379 👎 263
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let remain = n - k as usize;
        let mut score = card_points.iter().take(remain).sum::<i32>();
        let mut ans = score;
        for right in remain..n {
            score += card_points[right] - card_points[right - remain];
            ans = ans.min(score);
        }
        card_points.iter().sum::<i32>() - ans
    }
}

//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_score() {
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
        assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    }
}
