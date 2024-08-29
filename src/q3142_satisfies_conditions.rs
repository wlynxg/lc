//You are given a 2D matrix grid of size m x n. You need to check if each cell
//grid[i][j] is:
//
//
// Equal to the cell below it, i.e. grid[i][j] == grid[i + 1][j] (if it exists).
//
// Different from the cell to its right, i.e. grid[i][j] != grid[i][j + 1] (if
//it exists).
//
//
// Return true if all the cells satisfy these conditions, otherwise, return
//false.
//
//
// Example 1:
//
//
// Input: grid = [[1,0,2],[1,0,2]]
//
//
// Output: true
//
// Explanation:
//
//
//
// All the cells in the grid satisfy the conditions.
//
// Example 2:
//
//
// Input: grid = [[1,1,1],[0,0,0]]
//
//
// Output: false
//
// Explanation:
//
//
//
// All cells in the first row are equal.
//
// Example 3:
//
//
// Input: grid = [[1],[2],[3]]
//
//
// Output: false
//
// Explanation:
//
//
//
// Cells in the first column have different values.
//
//
// Constraints:
//
//
// 1 <= n, m <= 10
// 0 <= grid[i][j] <= 9
//
//
// Related Topics Array Matrix 👍 66 👎 0
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        grid.iter().enumerate().all(|(i, row)| {
            row.iter()
                .enumerate()
                .all(|(j, &v)| (j == 0 || v != row[j - 1]) && (i == 0 || v == grid[i - 1][j]))
        })
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q3142_satisfies_conditions::Solution;

    #[test]
    fn test() {
        assert_eq!(
            true,
            Solution::satisfies_conditions(vec![vec![1, 0, 2], vec![1, 0, 2]])
        );
        assert_eq!(
            false,
            Solution::satisfies_conditions(vec![vec![1, 1, 1], vec![0, 0, 0]])
        );
        assert_eq!(
            false,
            Solution::satisfies_conditions(vec![vec![1], vec![2], vec![3]])
        );
    }
}
