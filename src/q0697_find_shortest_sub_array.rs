//Given a non-empty array of non-negative integers nums, the degree of this
//array is defined as the maximum frequency of any one of its elements.
//
// Your task is to find the smallest possible length of a (contiguous) subarray
//of nums, that has the same degree as nums.
//
//
// Example 1:
//
//
//Input: nums = [1,2,2,3,1]
//Output: 2
//Explanation:
//The input array has a degree of 2 because both elements 1 and 2 appear twice.
//Of the subarrays that have the same degree:
//[1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
//The shortest length is 2. So return 2.
//
//
// Example 2:
//
//
//Input: nums = [1,2,2,3,1,4,2]
//Output: 6
//Explanation:
//The degree is 3 because the element 2 is repeated 3 times.
//So [2,2,3,1,4,2] is the shortest subarray, therefore returning 6.
//
//
//
// Constraints:
//
//
// nums.length will be between 1 and 50,000.
// nums[i] will be an integer between 0 and 49,999.
//
//
// Related Topics Array Hash Table 👍 3039 👎 1736

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();

        for (i, v) in nums.into_iter().enumerate() {
            let entry = map.entry(v).or_insert((0, i, i));
            entry.0 += 1;
            entry.2 = i;
        }

        let mut max_cnt = 0;
        let mut ret = 0;
        for (_, v) in map {
            if v.0 > max_cnt {
                max_cnt = v.0;
                ret = v.2 - v.1 + 1;
            } else if v.0 == max_cnt {
                ret = std::cmp::min(ret, v.2 - v.1 + 1);
            }
        }

        ret as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0697_find_shortest_sub_array::Solution;

    #[test]
    fn test() {
        assert_eq!(2, Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]));
        assert_eq!(
            6,
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2])
        );
    }
}
