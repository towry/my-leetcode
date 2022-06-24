/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 *
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (38.84%)
 * Likes:    7714
 * Dislikes: 0
 * Total Accepted:    1.8M
 * Total Submissions: 4.6M
 * Testcase Example:  '"abcabcbb"'
 *
 * Given a string s, find the length of the longest substring without repeating
 * characters.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not
 * a substring.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= s.length <= 5 * 10^4
 * s consists of English letters, digits, symbols and spaces.
 * 
 * 
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start_index: usize = 0;
        let mut cursor_index: usize = 0;
        let mut len: usize = 0;

        for c in s.chars() { 
            if let Some(slice) = s.get(start_index..cursor_index) {
                if let Some(index) = slice.find(c) {
                    // println!("{}, {}, {}", index, c, slice);
                    len = if (cursor_index - start_index) > len  { cursor_index - start_index } else { len };
                    start_index = start_index + index + 1;
                }
            }
            cursor_index += 1;
        }
        len = if (cursor_index - start_index) > len  { cursor_index - start_index } else { len };

        len as i32
    }
}
// @lc code=end

