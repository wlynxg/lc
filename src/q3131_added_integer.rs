//You are given two arrays of equal length, nums1 and nums2.
//
// Each element in nums1 has been increased (or decreased in the case of
//negative) by an integer, represented by the variable x.
//
// As a result, nums1 becomes equal to nums2. Two arrays are considered equal
//when they contain the same integers with the same frequencies.
//
// Return the integer x.
//
//
// Example 1:
//
//
// Input: nums1 = [2,6,4], nums2 = [9,7,5]
//
//
// Output: 3
//
// Explanation:
//
// The integer added to each element of nums1 is 3.
//
// Example 2:
//
//
// Input: nums1 = [10], nums2 = [5]
//
//
// Output: -5
//
// Explanation:
//
// The integer added to each element of nums1 is -5.
//
// Example 3:
//
//
// Input: nums1 = [1,1,1,1], nums2 = [1,1,1,1]
//
//
// Output: 0
//
// Explanation:
//
// The integer added to each element of nums1 is 0.
//
//
// Constraints:
//
//
// 1 <= nums1.length == nums2.length <= 100
// 0 <= nums1[i], nums2[i] <= 1000
// The test cases are generated in a way that there is an integer x such that
//nums1 can become equal to nums2 by adding x to each element of nums1.
//
//
// Related Topics Array 👍 112 👎 6

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        nums2.iter().max().unwrap() - nums1.iter().max().unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q3131_added_integer::Solution;

    #[test]
    fn test() {
        assert_eq!(3, Solution::added_integer(vec![2, 6, 4], vec![9, 7, 5]));
        assert_eq!(-5, Solution::added_integer(vec![10], vec![5]));
        assert_eq!(
            0,
            Solution::added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1])
        );
    }
}
