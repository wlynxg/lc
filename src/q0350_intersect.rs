//Given two integer arrays nums1 and nums2, return an array of their
//intersection. Each element in the result must appear as many times as it shows in both
//arrays and you may return the result in any order.
//
//
// Example 1:
//
//
//Input: nums1 = [1,2,2,1], nums2 = [2,2]
//Output: [2,2]
//
//
// Example 2:
//
//
//Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
//Output: [4,9]
//Explanation: [9,4] is also accepted.
//
//
//
// Constraints:
//
//
// 1 <= nums1.length, nums2.length <= 1000
// 0 <= nums1[i], nums2[i] <= 1000
//
//
//
// Follow up:
//
//
// What if the given array is already sorted? How would you optimize your
//algorithm?
// What if nums1's size is small compared to nums2's size? Which algorithm is
//better?
// What if elements of nums2 are stored on disk, and the memory is limited such
//that you cannot load all elements into the memory at once?
//
//
// Related Topics Array Hash Table Two Pointers Binary Search Sorting 👍 7681 👎
// 974

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for n in &nums1 {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut ret = vec![];
        for n in nums2 {
            if let Some(cnt) = map.get_mut(&n) {
                if *cnt > 0 {
                    *cnt -= 1;
                    ret.push(n);
                }
            }
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0350_intersect::Solution;

    #[test]
    fn test() {
        let result1 = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
        assert_eq!(2, result1.len());
        assert!(result1.contains(&2));
        assert!(result1.contains(&2));

        let result2 = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        assert_eq!(2, result2.len());
        assert!(result2.contains(&4));
        assert!(result2.contains(&9));
    }
}
