//You are given a 0-indexed array of positive integers nums.
//
// A subarray of nums is called incremovable if nums becomes strictly
//increasing on removing the subarray. For example, the subarray [3, 4] is an incremovable
//subarray of [5, 3, 4, 6, 7] because removing this subarray changes the array [5,
//3, 4, 6, 7] to [5, 6, 7] which is strictly increasing.
//
// Return the total number of incremovable subarrays of nums.
//
// Note that an empty array is considered strictly increasing.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
//
// Example 1:
//
//
//Input: nums = [1,2,3,4]
//Output: 10
//Explanation: The 10 incremovable subarrays are: [1], [2], [3], [4], [1,2], [2,
//3], [3,4], [1,2,3], [2,3,4], and [1,2,3,4], because on removing any one of
//these subarrays nums becomes strictly increasing. Note that you cannot select an
//empty subarray.
//
//
// Example 2:
//
//
//Input: nums = [6,5,7,8]
//Output: 7
//Explanation: The 7 incremovable subarrays are: [5], [6], [5,7], [6,5], [5,7,8]
//, [6,5,7] and [6,5,7,8].
//It can be shown that there are only 7 incremovable subarrays in nums.
//
//
// Example 3:
//
//
//Input: nums = [8,7,6,6]
//Output: 3
//Explanation: The 3 incremovable subarrays are: [8,7,6], [7,6,6], and [8,7,6,6]
//. Note that [8,7] is not an incremovable subarray because after removing [8,7]
//nums becomes [6,6], which is sorted in ascending order but not strictly
//increasing.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 50
// 1 <= nums[i] <= 50
//
//
// Related Topics Array Two Pointers Binary Search Enumeration 👍 138 👎 88

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut i = 0;
        while i < length - 1 && nums[i] < nums[i + 1] {
            i += 1;
        }

        if i == length - 1 {
            return (length * (length + 1)) as i32 >> 1;
        }

        let mut i = i as i32;
        let mut ans = i + 2;
        let mut j = length - 1;
        while j == length - 1 || nums[j] < nums[j + 1] {
            while i >= 0 && nums[i as usize] >= nums[j] {
                i -= 1;
            }

            ans += i + 2;
            j -= 1;
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q2970_incremovable_subarray_count::Solution;

    #[test]
    fn test() {
        assert_eq!(10, Solution::incremovable_subarray_count(vec![1, 2, 3, 4]));
        assert_eq!(7, Solution::incremovable_subarray_count(vec![6, 5, 7, 8]));
        assert_eq!(3, Solution::incremovable_subarray_count(vec![8, 7, 6, 6]));
    }
}
