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
 * 3   7   11
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
 * 1     7      13 .   .  19
 * 2   6 8 . 12 14  .  18 20
 * 3 5   9 11   15  17    21
 * 4 .  10      16        22
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

/*
1 . . . . . 13
2 . . . . 12
3 . . . 11
4 . . 10
5 . 9
6 8
7
*/

// @lc code=start
impl Solution {
    fn get_column_size(num_rows: usize, len: usize) -> usize {
        (len - len % num_rows) / 2
    }

    pub fn convert(s: String, num_rows: i32) -> String {
        let middle_size: usize = num_rows as usize - 2;
        let col_size = Solution::get_column_size(num_rows as usize, s.len());
        let chars = s.chars().collect::<Vec<_>>();

        let mut current_middle_size = middle_size;

        println!("{}", col_size);

        for r in 0..num_rows as usize {
            let mut col_index: usize = 0;
            // each rows.
            let start_value = r;

            while col_index < col_size {
                // col by col
                let mut char_index = start_value + 2 * col_index;
                println!(
                    "{}, {}, {}",
                    chars.get(char_index).unwrap_or(&'-'),
                    char_index,
                    col_index
                );

                col_index += 1;

                // num in middle.
                col_index += current_middle_size;
                char_index = start_value + 2 * col_index;
                if current_middle_size >= middle_size {
                    continue;
                }
                println!(
                    "{}, {}, {}",
                    chars.get(char_index).unwrap_or(&'-'),
                    char_index,
                    col_index
                );
                // skip the middle area.
                col_index += (middle_size - 1);
            }

            current_middle_size -= 1;
            if current_middle_size < 0 {
                current_middle_size = middle_size;
            }
        }

        String::from("")
    }
}
// @lc code=end
