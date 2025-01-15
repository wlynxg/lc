//Let the function f(s) be the frequency of the lexicographically smallest
//character in a non-empty string s. For example, if s = "dcce" then f(s) = 2 because
//the lexicographically smallest character is 'c', which has a frequency of 2.
//
// You are given an array of strings words and another array of query strings
//queries. For each query queries[i], count the number of words in words such that
//f(queries[i]) < f(W) for each W in words.
//
// Return an integer array answer, where each answer[i] is the answer to the iᵗʰ
// query.
//
//
// Example 1:
//
//
//Input: queries = ["cbd"], words = ["zaaaz"]
//Output: [1]
//Explanation: On the first query we have f("cbd") = 1, f("zaaaz") = 3 so f(
//"cbd") < f("zaaaz").
//
//
// Example 2:
//
//
//Input: queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
//Output: [1,2]
//Explanation: On the first query only f("bbb") < f("aaaa"). On the second
//query both f("aaa") and f("aaaa") are both > f("cc").
//
//
//
// Constraints:
//
//
// 1 <= queries.length <= 2000
// 1 <= words.length <= 2000
// 1 <= queries[i].length, words[i].length <= 10
// queries[i][j], words[i][j] consist of lowercase English letters.
//
//
// Related Topics Array Hash Table String Binary Search Sorting 👍 721 👎 976
pub struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut counts = vec![0; 12];
        let mut ans = vec![0; queries.len()];

        for word in words.iter() {
            counts[Self::count(word) as usize] += 1;
        }

        for i in (1..11).rev() {
            counts[i] += counts[i + 1];
        }

        for i in 0..queries.len() {
            ans[i] = counts[Self::count(&queries[i]) as usize + 1];
        }
        ans
    }

    fn count(s: &String) -> i32 {
        let chars = s.bytes().collect::<Vec<u8>>();
        let mut min_byte = 'z' as u8;
        let mut ans = 0;
        for i in 0..chars.len() {
            if chars[i] < min_byte {
                min_byte = chars[i];
                ans = 1;
            } else if chars[i] == min_byte {
                ans += 1;
            }
        }
        ans
    }
}
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_num_smaller_by_frequency() {
        // assert_eq!(
        //     Solution::num_smaller_by_frequency(vec!["cbd".to_string()], vec!["zaaaz".to_string()]),
        //     vec![1]
        // );
        // assert_eq!(
        //     Solution::num_smaller_by_frequency(
        //         vec!["bbb".to_string(), "cc".to_string()],
        //         vec![
        //             "a".to_string(),
        //             "aa".to_string(),
        //             "aaa".to_string(),
        //             "aaaa".to_string()
        //         ]
        //     ),
        //     vec![1, 2]
        // );
        assert_eq!(
            Solution::num_smaller_by_frequency(
                vec![
                    "bba".to_string(),
                    "abaaaaaa".to_string(),
                    "aaaaaa".to_string(),
                    "bbabbabaab".to_string(),
                    "aba".to_string(),
                    "aa".to_string(),
                    "baab".to_string(),
                    "bbbbbb".to_string(),
                    "aab".to_string(),
                    "bbabbaabb".to_string(),
                ],
                vec![
                    "aaabbb".to_string(),
                    "aab".to_string(),
                    "babbab".to_string(),
                    "babbbb".to_string(),
                    "b".to_string(),
                    "bbbbbbbbab".to_string(),
                    "a".to_string(),
                    "bbbbbbbbbb".to_string(),
                    "baaabbaab".to_string(),
                    "aa".to_string(),
                ]
            ),
            vec![6, 1, 1, 2, 3, 3, 3, 1, 3, 2]
        );
    }
}
