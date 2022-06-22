/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] ZigZag Conversion
 *
 * https://leetcode.cn/problems/zigzag-conversion/description/
 *
 * algorithms
 * Medium (51.95%)
 * Likes:    1707
 * Dislikes: 0
 * Total Accepted:    447.5K
 * Total Submissions: 861.5K
 * Testcase Example:  '"PAYPALISHIRING"\n3'
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
 * of rows like this: (you may want to display this pattern in a fixed font for
 * better legibility)
 *
 * 1 . 5 .  9  . 13
 * 2 4 6 8 10 12 14
 * 3   7   11    15
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a
 * number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * 1     7      13
 * 2   6 8 . 12 14
 * 3 5   9 .11
 * 4 .   10
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 *
 * Example 3:
 *
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consists of English letters (lower-case and upper-case), ',' and '.'.
 * 1 <= numRows <= 1000
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let middle_size = num_rows - 2;
    }
}
// @lc code=end
