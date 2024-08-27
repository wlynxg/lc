//Given an integer array arr, return true if there are three consecutive odd
//numbers in the array. Otherwise, return false.
//
//
// Example 1:
//
//
//Input: arr = [2,6,4,1]
//Output: false
//Explanation: There are no three consecutive odds.
//
//
// Example 2:
//
//
//Input: arr = [1,2,34,3,4,5,7,23,12]
//Output: true
//Explanation: [5,7,23] are three consecutive odds.
//
//
//
// Constraints:
//
//
// 1 <= arr.length <= 1000
// 1 <= arr[i] <= 1000
//
//
// Related Topics Array 👍 1133 👎 94
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut cnt = 0;
        for n in arr {
            if n % 2 == 1 {
                cnt += 1;
            } else {
                cnt = 0;
            }

            if cnt == 3 {
                return true;
            }
        }
        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q1550_three_consecutive_odds::Solution;

    #[test]
    fn test() {
        assert_eq!(false, Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
        assert_eq!(
            true,
            Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12])
        );
    }
}
