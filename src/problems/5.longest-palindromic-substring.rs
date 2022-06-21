/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 *
 * https://leetcode.cn/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (36.79%)
 * Likes:    5360
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 2.9M
 * Testcase Example:  '"babad"'
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consist of only digits and English letters.
 *
 *
 */

// @lc code=start
impl Solution {
    fn is_palindrome(s: &str) -> bool {
        let half = s.len() / 2;
        return s.chars().take(half).eq(s.chars().rev().take(half));
    }
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();

        let mut window_size = len;
        let mut left_cursor = 0;
        let mut right_cursor = left_cursor + window_size;

        if s.get(left_cursor..right_cursor)
            .map_or(false, |s| Solution::is_palindrome(s))
        {
            return s.get(left_cursor..right_cursor).unwrap().to_owned();
        }

        window_size -= 1;
        right_cursor = left_cursor + window_size;

        while window_size > 1 {
            // println!("{}, {}", left_cursor, window_size);
            while left_cursor + window_size <= len {
                // println!("{:?}", s.get(left_cursor..));
                if s.get(left_cursor..right_cursor)
                    .map_or(false, |s| Solution::is_palindrome(s))
                {
                    return s.get(left_cursor..right_cursor).unwrap().to_owned();
                }
                left_cursor += 1;
                right_cursor = left_cursor + window_size;
            }
            window_size -= 1;
            left_cursor = 0;
            right_cursor = left_cursor + window_size;
        }

        s.get(0..1).map_or(String::from(""), |s| s.to_owned())
    }
}
// @lc code=end
