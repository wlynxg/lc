//Given two binary strings a and b, return their sum as a binary string.
//
//
// Example 1:
// Input: a = "11", b = "1"
//Output: "100"
//
// Example 2:
// Input: a = "1010", b = "1011"
//Output: "10101"
//
//
// Constraints:
//
//
// 1 <= a.length, b.length <= 10⁴
// a and b consist only of '0' or '1' characters.
// Each string does not contain leading zeros except for the zero itself.
//
//
// Related Topics Math String Bit Manipulation Simulation 👍 9418 👎 976

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut a_last = a.len() as i32 - 1;
        let mut b_last = b.len() as i32 - 1;
        let mut result = vec![];
        let mut carry = 0;

        while a_last > -1 || b_last > -1 {
            let ba = *a.get(a_last as usize).unwrap_or(&b'0') - b'0';
            let bb = *b.get(b_last as usize).unwrap_or(&b'0') - b'0';

            let mut sum = ba + bb + carry;
            carry = sum >> 1;
            sum %= 2;
            result.insert(0, b'0' + sum);

            a_last -= 1;
            b_last -= 1;
        }

        if carry > 0 {
            result.insert(0, b'1');
        }

        String::from_utf8(result).unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0067_add_binary::Solution;

    #[test]
    fn test() {
        assert_eq!(
            String::from("100"),
            Solution::add_binary(String::from("11"), String::from("1"))
        );
        assert_eq!(
            String::from("10101"),
            Solution::add_binary(String::from("1010"), String::from("1011"))
        );
    }
}
