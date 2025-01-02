//According to Wikipedia's article: "The Game of Life, also known simply as
//Life, is a cellular automaton devised by the British mathematician John Horton
//Conway in 1970."
//
// The board is made up of an m x n grid of cells, where each cell has an
//initial state: live (represented by a 1) or dead (represented by a 0). Each cell
//interacts with its eight neighbors (horizontal, vertical, diagonal) using the
//following four rules (taken from the above Wikipedia article):
//
//
// Any live cell with fewer than two live neighbors dies as if caused by under-
//population.
// Any live cell with two or three live neighbors lives on to the next
//generation.
// Any live cell with more than three live neighbors dies, as if by over-
//population.
// Any dead cell with exactly three live neighbors becomes a live cell, as if
//by reproduction.
//
//
// The next state of the board is determined by applying the above rules
//simultaneously to every cell in the current state of the m x n grid board. In this
//process, births and deaths occur simultaneously.
//
// Given the current state of the board, update the board to reflect its next
//state.
//
// Note that you do not need to return anything.
//
//
// Example 1:
//
//
//Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
//Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
//
//
// Example 2:
//
//
//Input: board = [[1,1],[1,0]]
//Output: [[1,1],[1,1]]
//
//
//
// Constraints:
//
//
// m == board.length
// n == board[i].length
// 1 <= m, n <= 25
// board[i][j] is 0 or 1.
//
//
//
// Follow up:
//
//
// Could you solve it in-place? Remember that the board needs to be updated
//simultaneously: You cannot update some cells first and then use their updated
//values to update other cells.
// In this question, we represent the board using a 2D array. In principle, the
//board is infinite, which would cause problems when the active area encroaches
//upon the border of the array (i.e., live cells reach the border). How would you
//address these problems?
//
//
// Related Topics Array Matrix Simulation 👍 6487 👎 587
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    const DIRS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        // 2 : 1 -> 0
        // 3 : 0 -> 1
        let (m, n) = (board.len(), board[0].len());

        for i in 0..m {
            for j in 0..n {
                let mut ret = 0;
                for (dx, dy) in Self::DIRS {
                    let x1 = i as i32 + dx;
                    let y1 = j as i32 + dy;
                    if x1 < 0
                        || x1 >= m as i32
                        || y1 < 0
                        || y1 >= n as i32
                        || board[x1 as usize][y1 as usize] == 0
                        || board[x1 as usize][y1 as usize] == 3
                    {
                        continue;
                    }
                    ret += 1;
                }

                let current = board[i][j];
                // Any live cell with fewer than two live neighbors dies as if caused by under-population.
                // Any live cell with two or three live neighbors lives on to the next generation.
                // Any live cell with more than three live neighbors dies, as if by over-population.
                // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                if current == 1 && ret < 2 {
                    board[i][j] = 2;
                } else if current == 1 && (ret == 2 || ret == 3) {
                    // board[i][j] = 1;
                } else if current == 1 && ret > 3 {
                    board[i][j] = 2;
                } else if current == 0 && ret == 3 {
                    board[i][j] = 3;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 2 {
                    board[i][j] = 0
                } else if board[i][j] == 3 {
                    board[i][j] = 1
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_of_life() {
        let mut board1 = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board1);
        assert_eq!(
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
            board1
        );

        let mut board2 = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut board2);
        assert_eq!(vec![vec![1, 1], vec![1, 1]], board2);
    }
}
