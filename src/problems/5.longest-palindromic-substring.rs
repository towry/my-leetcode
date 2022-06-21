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
        let mut current_max_len: usize = 0;
        let mut left_cursor: usize = 0;
        let mut right_cursor: usize = s.len();
        while left_cursor != right_cursor {
            if s.get(left_cursor..right_cursor)
                .map_or(false, |s| Solution::is_palindrome(s))
            {
                let len = right_cursor - left_cursor;
                current_max_len = if len > current_max_len {
                    len
                } else {
                    current_max_len
                };
                break;
            }
            left_cursor += 1;
            right_cursor -= 1;
        }

        left_cursor = 0;
        right_cursor = s.len() - 1;
        while left_cursor != right_cursor && right_cursor + 1 - left_cursor > current_max_len {
            if s.get(left_cursor..right_cursor)
                .map_or(false, |s| Solution::is_palindrome(s))
            {
                current_max_len = right_cursor - left_cursor;
                break;
            }
            left_cursor += 1;
        }

        left_cursor = 0;
        right_cursor = s.len();
        while left_cursor != right_cursor && right_cursor + 1 - left_cursor > current_max_len {
            if s.get(left_cursor..right_cursor)
                .map_or(false, |s| Solution::is_palindrome(s))
            {
                current_max_len = right_cursor - left_cursor;
                break;
            }
            right_cursor -= 1;
        }

        current_max_len
    }
}
// @lc code=end
