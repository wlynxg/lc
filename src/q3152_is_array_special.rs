//An array is considered special if every pair of its adjacent elements
//contains two numbers with different parity.
//
// You are given an array of integer nums and a 2D integer matrix queries,
//where for queries[i] = [fromi, toi] your task is to check that subarray nums[fromi..
//toi] is special or not.
//
// Return an array of booleans answer such that answer[i] is true if nums[fromi.
//.toi] is special.
//
//
//
// Example 1:
//
//
// Input: nums = [3,4,1,2,6], queries = [[0,4]]
//
//
// Output: [false]
//
// Explanation:
//
// The subarray is [3,4,1,2,6]. 2 and 6 are both even.
//
// Example 2:
//
//
// Input: nums = [4,3,1,6], queries = [[0,2],[2,3]]
//
//
// Output: [false,true]
//
// Explanation:
//
//
// The subarray is [4,3,1]. 3 and 1 are both odd. So the answer to this query
//is false.
// The subarray is [1,6]. There is only one pair: (1,6) and it contains numbers
//with different parity. So the answer to this query is true.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// 1 <= nums[i] <= 10⁵
// 1 <= queries.length <= 10⁵
// queries[i].length == 2
// 0 <= queries[i][0] <= queries[i][1] <= nums.length - 1
//
//
// Related Topics Array Binary Search Prefix Sum 👍 179 👎 12

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut s: Vec<usize> = vec![0; nums.len()];
        let mut ret = vec![false; queries.len()];

        for i in 1..nums.len() {
            s[i] = s[i - 1];
            if nums[i] % 2 == nums[i - 1] % 2 {
                s[i] += 1;
            }
        }

        for (i, v) in queries.iter().enumerate() {
            ret[i] = s[v[0] as usize] == s[v[1] as usize]
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q3152_is_array_special::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![false],
            Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]])
        );
        assert_eq!(
            vec![false, true],
            Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]])
        );
    }
}
