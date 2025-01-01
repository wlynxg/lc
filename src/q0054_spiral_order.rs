//Given an m x n matrix, return all elements of the matrix in spiral order.
//
//
// Example 1:
//
//
//Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//Output: [1,2,3,6,9,8,7,4,5]
//
//
// Example 2:
//
//
//Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
//Output: [1,2,3,4,8,12,11,10,9,5,6,7]
//
//
//
// Constraints:
//
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 10
// -100 <= matrix[i][j] <= 100
//
//
// Related Topics Array Matrix Simulation 👍 15453 👎 1384

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut m, mut n) = (matrix.len(), matrix[0].len());
        let size = m * n;
        let mut result: Vec<i32> = Vec::with_capacity(size);
        let (mut i, mut j, mut di) = (0, -1, 0);

        while result.len() < size {
            let (dx, dy) = Self::DIRS[di as usize];
            for _ in 0..n {
                i += dx;
                j += dy;
                result.push(matrix[i as usize][j as usize]);
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
    use super::Solution;
    #[test]
    fn test_spiral_order() {
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ])
        );
    }
}
