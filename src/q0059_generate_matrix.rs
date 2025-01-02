//Given a positive integer n, generate an n x n matrix filled with elements
//from 1 to n² in spiral order.
//
//
// Example 1:
//
//
//Input: n = 3
//Output: [[1,2,3],[8,9,4],[7,6,5]]
//
//
// Example 2:
//
//
//Input: n = 1
//Output: [[1]]
//
//
//
// Constraints:
//
//
// 1 <= n <= 20
//
//
// Related Topics Array Matrix Simulation 👍 6536 👎 268

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];
        let (mut n, mut m) = (n, n);
        let mut value = 1;
        let (mut x, mut y, mut di) = (0, -1, 0);

        while n > 0 {
            let (dx, dy) = Self::DIRS[di];
            for _ in 0..n {
                x += dx;
                y += dy;
                result[x as usize][y as usize] = value;
                value += 1;
            }
            di = (di + 1) % 4;
            (n, m) = (m - 1, n);
        }

        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_matrix() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            Solution::generate_matrix(3)
        );
        assert_eq!(vec![vec![1]], Solution::generate_matrix(1));
    }
}
