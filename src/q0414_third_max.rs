//Given an integer array nums, return the third distinct maximum number in this
//array. If the third maximum does not exist, return the maximum number.
//
//
// Example 1:
//
//
//Input: nums = [3,2,1]
//Output: 1
//Explanation:
//The first distinct maximum is 3.
//The second distinct maximum is 2.
//The third distinct maximum is 1.
//
//
// Example 2:
//
//
//Input: nums = [1,2]
//Output: 2
//Explanation:
//The first distinct maximum is 2.
//The second distinct maximum is 1.
//The third distinct maximum does not exist, so the maximum (2) is returned
//instead.
//
//
// Example 3:
//
//
//Input: nums = [2,2,3,1]
//Output: 1
//Explanation:
//The first distinct maximum is 3.
//The second distinct maximum is 2 (both 2's are counted together since they
//have the same value).
//The third distinct maximum is 1.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -2³¹ <= nums[i] <= 2³¹ - 1
//
//
//
//Follow up: Can you find an
//O(n) solution?
//
// Related Topics Array Sorting 👍 3037 👎 3179

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut a = i64::MIN;
        let (mut b, mut c) = (a, a);
        for i in nums {
            let i = i as i64;
            if i > a {
                if c != b {
                    c = b
                }
                if b != a {
                    b = a
                }
                a = i;
            } else if i > b {
                if a == i {
                    continue;
                }
                if c != b {
                    c = b
                }
                b = i;
            } else if i > c {
                if b == i {
                    continue;
                }
                c = i;
            }
        }
        if c == i64::MIN {
            a as i32
        } else {
            c as i32
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0414_third_max::Solution;

    #[test]
    fn test() {
        assert_eq!(1, Solution::third_max(vec![3, 2, 1]));
        assert_eq!(2, Solution::third_max(vec![1, 2]));
        assert_eq!(1, Solution::third_max(vec![2, 2, 3, 1]));
    }
}
