//Roman numerals are represented by seven different symbols: I, V, X, L, C, D
//and M.
//
//
//Symbol       Value
//I             1
//V             5
//X             10
//L             50
//C             100
//D             500
//M             1000
//
// For example, 2 is written as II in Roman numeral, just two ones added
//together. 12 is written as XII, which is simply X + II. The number 27 is written as
//XXVII, which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left to right.
//However, the numeral for four is not IIII. Instead, the number four is written as
//IV. Because the one is before the five we subtract it making four. The same
//principle applies to the number nine, which is written as IX. There are six
//instances where subtraction is used:
//
//
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
//
//
// Given a roman numeral, convert it to an integer.
//
//
// Example 1:
//
//
//Input: s = "III"
//Output: 3
//Explanation: III = 3.
//
//
// Example 2:
//
//
//Input: s = "LVIII"
//Output: 58
//Explanation: L = 50, V= 5, III = 3.
//
//
// Example 3:
//
//
//Input: s = "MCMXCIV"
//Output: 1994
//Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].
//
//
// Related Topics Hash Table Math String 👍 14277 👎 944

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .fold((0, ' '), |res, c| match (res.1, c) {
                ('I', 'V') => (res.0 + 3, 'V'),
                ('I', 'X') => (res.0 + 8, 'X'),
                ('X', 'L') => (res.0 + 30, 'L'),
                ('X', 'C') => (res.0 + 80, 'C'),
                ('C', 'D') => (res.0 + 300, 'D'),
                ('C', 'M') => (res.0 + 800, 'M'),
                (_, 'I') => (res.0 + 1, 'I'),
                (_, 'V') => (res.0 + 5, 'V'),
                (_, 'X') => (res.0 + 10, 'X'),
                (_, 'L') => (res.0 + 50, 'L'),
                (_, 'C') => (res.0 + 100, 'C'),
                (_, 'D') => (res.0 + 500, 'D'),
                (_, 'M') => (res.0 + 1000, 'M'),
                (_, _) => unreachable!(),
            })
            .0
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0013_roman_to_int::Solution;

    #[test]
    fn test() {
        assert_eq!(3, Solution::roman_to_int(String::from("III")));
        assert_eq!(58, Solution::roman_to_int(String::from("LVIII")));
        assert_eq!(1994, Solution::roman_to_int(String::from("MCMXCIV")));
    }
}
