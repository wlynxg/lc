//A self-dividing number is a number that is divisible by every digit it
//contains.
//
//
// For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0
//, and 128 % 8 == 0.
//
//
// A self-dividing number is not allowed to contain the digit zero.
//
// Given two integers left and right, return a list of all the self-dividing
//numbers in the range [left, right].
//
//
// Example 1:
// Input: left = 1, right = 22
//Output: [1,2,3,4,5,6,7,8,9,11,12,15,22]
//
// Example 2:
// Input: left = 47, right = 85
//Output: [48,55,66,77]
//
//
// Constraints:
//
//
// 1 <= left <= right <= 10⁴
//
//
// Related Topics Math 👍 1785 👎 375
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        (left..=right)
            .filter(|&i| {
                let mut cur = i;
                while cur > 0 {
                    let t = cur % 10;
                    if t == 0 || i % t != 0 {
                        return false;
                    }
                    cur /= 10;
                }
                true
            })
            .collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0728_self_dividing_numbers::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22],
            Solution::self_dividing_numbers(1, 22)
        );
        assert_eq!(
            vec![48, 55, 66, 77],
            Solution::self_dividing_numbers(47, 85)
        );
    }
}
