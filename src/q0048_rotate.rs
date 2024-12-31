//You are given an n x n 2D matrix representing an image, rotate the image by 90
// degrees (clockwise).
//
// You have to rotate the image in-place, which means you have to modify the
//input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
//
//
// Example 1:
//
//
//Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//Output: [[7,4,1],[8,5,2],[9,6,3]]
//
//
// Example 2:
//
//
//Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
//Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
//
//
//
// Constraints:
//
//
// n == matrix.length == matrix[i].length
// 1 <= n <= 20
// -1000 <= matrix[i][j] <= 1000
//
//
// Related Topics Array Math Matrix 👍 18178 👎 866

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return;
        }

        let m = matrix.len();
        let last = m - 1;

        for i in 0..last {
            for j in 0..last - i {
                (matrix[i][j], matrix[last - j][last - i]) =
                    (matrix[last - j][last - i], matrix[i][j]);
            }
        }

        for i in 0..m / 2 {
            for j in 0..m {
                (matrix[i][j], matrix[last - i][j]) = (matrix[last - i][j], matrix[i][j]);
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q0048_rotate::Solution;

    #[test]
    fn test_solution() {
        // Example 1
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);

        // Example 2
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let expected = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
