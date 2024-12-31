//Given an m x n matrix mat, return an array of all the elements of the array
//in a diagonal order.
//
//
// Example 1:
//
//
//Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
//Output: [1,2,4,7,5,3,6,8,9]
//
//
// Example 2:
//
//
//Input: mat = [[1,2],[3,4]]
//Output: [1,2,3,4]
//
//
//
// Constraints:
//
//
// m == mat.length
// n == mat[i].length
// 1 <= m, n <= 10⁴
// 1 <= m * n <= 10⁴
// -10⁵ <= mat[i][j] <= 10⁵
//
//
// Related Topics Array Matrix Simulation 👍 3558 👎 709

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)

// 每一趟对角线中元素的坐标（x, y）相加的和是递增的。
// 第一趟：1 的坐标(0, 0)。x + y == 0。
// 第二趟：2 的坐标(1, 0)，4 的坐标(0, 1)。x + y == 1。
// 第三趟：7 的坐标(0, 2), 5 的坐标(1, 1)，3 的坐标(2, 0)。第三趟 x + y == 2。
// 第四趟：……
//
// 每一趟都是 x 或 y 其中一个从大到小（每次-1），另一个从小到大（每次+1）。
// 第二趟：2 的坐标(1, 0)，4 的坐标(0, 1)。x 每次-1，y 每次+1。
// 第三趟：7 的坐标(0, 2), 5 的坐标(1, 1)，3 的坐标(2, 0)。x 每次 +1，y 每次 -1。
//
// 确定初始值。例如这一趟是 x 从大到小， x 尽量取最大，当初始值超过 x 的上限时，不足的部分加到 y 上面。
// 第二趟：2 的坐标(1, 0)，4 的坐标(0, 1)。x + y == 1，x 初始值取 1，y 取 0。
// 第四趟：6 的坐标(2, 1)，8 的坐标(1, 2)。x + y == 3，x 初始值取 2，剩下的加到 y上，y 取 1。
//
// 确定结束值。例如这一趟是 x 从大到小，这一趟结束的判断是， x 减到 0 或者 y 加到上限。
// 第二趟：2 的坐标(1, 0)，4 的坐标(0, 1)。x 减到 0 为止。
// 第四趟：6 的坐标(2, 1)，8 的坐标(1, 2)。x 虽然才减到 1，但是 y 已经加到上限了。
//
// 这一趟是 x 从大到小，那么下一趟是 y 从大到小，循环进行。 并且方向相反时，逻辑处理是一样的，除了x，y和他们各自的上限值是相反的。
// x 从大到小，第二趟：2 的坐标(1, 0)，4 的坐标(0, 1)。x + y == 1，x 初始值取 1，y 取 0。结束值 x 减到 0 为止。
// x 从小到大，第三趟：7 的坐标(0, 2)，5 的坐标(1, 1)，3 的坐标(2, 0)。x + y == 2，y 初始值取 2，x 取 0。结束值 y 减到 0 为止。
//
// 作者：Ikaruga
// 链接：https://leetcode.cn/problems/diagonal-traverse/solutions/11440/dui-jiao-xian-bian-li-fen-xi-ti-mu-zhao-zhun-gui-l/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        if m <= 0 {
            return Vec::new();
        }

        let n = mat[0].len();
        if n <= 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(m * n);
        let mut i = 0;
        while i < (m + n) {
            let mut x1 = if i < m { i } else { m - 1 };
            let mut y1 = i - x1;

            while y1 < n {
                result.push(mat[x1][y1]);
                if x1 == 0 {
                    break;
                }
                x1 -= 1;
                y1 += 1;
            }
            i += 1;

            if i >= m + n {
                break;
            }
            let mut y2 = if i < n { i } else { n - 1 };
            let mut x2 = i - y2;
            while x2 < m {
                result.push(mat[x2][y2]);
                x2 += 1;
                if y2 == 0 {
                    break;
                }
                y2 -= 1;
            }
            i += 1;
        }

        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_diagonal_order() {
        // Example 1
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];
        assert_eq!(Solution::find_diagonal_order(mat), expected);

        // Example 2
        let mat = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_diagonal_order(mat), expected);
    }
}
