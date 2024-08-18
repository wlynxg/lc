//Write a function that takes the binary representation of a positive integer
//and returns the number of set bits it has (also known as the Hamming weight).
//
//
// Example 1:
//
//
// Input: n = 11
//
//
// Output: 3
//
// Explanation:
//
// The input binary string 1011 has a total of three set bits.
//
// Example 2:
//
//
// Input: n = 128
//
//
// Output: 1
//
// Explanation:
//
// The input binary string 10000000 has a total of one set bit.
//
// Example 3:
//
//
// Input: n = 2147483645
//
//
// Output: 30
//
// Explanation:
//
// The input binary string 1111111111111111111111111111101 has a total of
//thirty set bits.
//
//
// Constraints:
//
//
// 1 <= n <= 2³¹ - 1
//
//
//
//Follow up: If this function is called many times, how would you optimize it?
//
// Related Topics Divide and Conquer Bit Manipulation 👍 6542 👎 1340
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut ret = 0;
        let mut n = n;
        while n > 0 {
            ret += 1;
            n = n & (n - 1);
        }

        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0191_hamming_weight::Solution;

    #[test]
    fn test() {
        assert_eq!(3, Solution::hamming_weight(11));
        assert_eq!(1, Solution::hamming_weight(128));
        assert_eq!(30, Solution::hamming_weight(2147483645));
    }
}
