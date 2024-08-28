//You are given a 2D integer array logs where each logs[i] = [birthi, deathi]
//indicates the birth and death years of the iᵗʰ person.
//
// The population of some year x is the number of people alive during that year.
// The iᵗʰ person is counted in year x's population if x is in the inclusive
//range [birthi, deathi - 1]. Note that the person is not counted in the year that
//they die.
//
// Return the earliest year with the maximum population.
//
//
// Example 1:
//
//
//Input: logs = [[1993,1999],[2000,2010]]
//Output: 1993
//Explanation: The maximum population is 1, and 1993 is the earliest year with
//this population.
//
//
// Example 2:
//
//
//Input: logs = [[1950,1961],[1960,1971],[1970,1981]]
//Output: 1960
//Explanation:
//The maximum population is 2, and it had happened in years 1960 and 1970.
//The earlier year between them is 1960.
//
//
// Constraints:
//
//
// 1 <= logs.length <= 100
// 1950 <= birthi < deathi <= 2050
//
//
// Related Topics Array Counting Prefix Sum 👍 1315 👎 236
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut ret = 1950;

        for year in 1950..=2050 {
            let population = logs
                .iter()
                .filter(|&log| (log[0]..log[1]).contains(&year))
                .count();

            if population > max {
                max = population;
                ret = year;
            }
        }

        ret
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use crate::q1854_maximum_population::Solution;

    #[test]
    fn test() {
        assert_eq!(
            1993,
            Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]])
        );
        assert_eq!(
            1960,
            Solution::maximum_population(vec![
                vec![1950, 1961],
                vec![1960, 1971],
                vec![1970, 1981]
            ])
        );
    }
}
