//You are visiting a farm that has a single row of fruit trees arranged from
//left to right. The trees are represented by an integer array fruits where fruits[i]
// is the type of fruit the iᵗʰ tree produces.
//
// You want to collect as much fruit as possible. However, the owner has some
//strict rules that you must follow:
//
//
// You only have two baskets, and each basket can only hold a single type of
//fruit. There is no limit on the amount of fruit each basket can hold.
// Starting from any tree of your choice, you must pick exactly one fruit from
//every tree (including the start tree) while moving to the right. The picked
//fruits must fit in one of your baskets.
// Once you reach a tree with fruit that cannot fit in your baskets, you must
//stop.
//
//
// Given the integer array fruits, return the maximum number of fruits you can
//pick.
//
//
// Example 1:
//
//
//Input: fruits = [1,2,1]
//Output: 3
//Explanation: We can pick from all 3 trees.
//
//
// Example 2:
//
//
//Input: fruits = [0,1,2,2]
//Output: 3
//Explanation: We can pick from trees [1,2,2].
//If we had started at the first tree, we would only pick from trees [0,1].
//
//
// Example 3:
//
//
//Input: fruits = [1,2,3,2,2]
//Output: 4
//Explanation: We can pick from trees [2,3,2,2].
//If we had started at the first tree, we would only pick from trees [1,2].
//
//
//
// Constraints:
//
//
// 1 <= fruits.length <= 10⁵
// 0 <= fruits[i] < fruits.length
//
//
// Related Topics Array Hash Table Sliding Window 👍 4632 👎 343

pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let (mut first, mut second) = (-1, -1);
        let (mut start, mut index) = (0, 0);
        let mut max_length = 0;

        for i in 0..fruits.len() {
            if fruits[i] != first && fruits[i] != second {
                first = fruits[index];
                second = fruits[i];
                start = index;
            }
            if fruits[i] != fruits[index] {
                index = i;
            }

            max_length = max_length.max(i - start + 1);
        }

        return max_length as i32;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::total_fruit(vec![1, 2, 1]));
        assert_eq!(3, Solution::total_fruit(vec![0, 1, 2, 2]));
        assert_eq!(4, Solution::total_fruit(vec![1, 2, 3, 2, 2]));
        assert_eq!(
            5,
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4])
        );
        assert_eq!(5, Solution::total_fruit(vec![1, 0, 1, 4, 1, 4, 1, 2, 3]));
    }
}
