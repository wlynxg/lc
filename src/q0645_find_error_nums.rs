//You have a set of integers s, which originally contains all the numbers from 1
// to n. Unfortunately, due to some error, one of the numbers in s got duplicated
//to another number in the set, which results in repetition of one number and
//loss of another number.
//
// You are given an integer array nums representing the data status of this set
//after the error.
//
// Find the number that occurs twice and the number that is missing and return
//them in the form of an array.
//
//
// Example 1:
// Input: nums = [1,2,2,4]
//Output: [2,3]
//
// Example 2:
// Input: nums = [1,1]
//Output: [1,2]
//
//
// Constraints:
//
//
// 2 <= nums.length <= 10⁴
// 1 <= nums[i] <= 10⁴
//
//
// Related Topics Array Hash Table Bit Manipulation Sorting 👍 4750 👎 1163
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut array = vec![0; nums.len() + 1];
        let mut ret = vec![0; 2];

        for num in nums {
            array[num as usize] += 1;
        }

        for (i, &c) in array.iter().enumerate() {
            if c == 2 {
                ret[0] = i as i32;
            } else if c == 0 {
                ret[1] = i as i32;
            }
        }
        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0645_find_error_nums::Solution;

    #[test]
    fn test() {
        assert_eq!(vec![2, 3], Solution::find_error_nums(vec![1, 2, 2, 4]));
        assert_eq!(vec![1, 2], Solution::find_error_nums(vec![1, 1]));
    }
}
