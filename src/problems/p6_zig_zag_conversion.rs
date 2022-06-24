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
 * 0 . 4 .  8  . 12
 * 1 3 5 7 9 11 13
 * 2   6   10    14
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
 * 0     6      12
 * 1   5 7 . 11 13
 * 2 4   8 10
 * 3 .  9
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
#![deny(arithmetic_overflow)]

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn get_column_size(num_rows: usize, len: usize) -> usize {
        let mut mut_len = len;
        let mut count: usize = 0;
        let step = if num_rows <= 1 { 1 } else { 2 };
        while mut_len >= step {
            count += 1;
            mut_len -= step;
            if mut_len <= 0 {
                return count;
            }
        }

        count + 1
    }

    pub fn convert(s: String, num_rows: i32) -> String {
        let middle_size: usize = if num_rows > 2 {
            num_rows as usize - 2
        } else {
            0
        };
        if s.len() <= 0 {
            return s;
        }
        let col_size = Solution::get_column_size(num_rows as usize, s.len());
        let chars = s.chars().collect::<Vec<_>>();
        let mut result: Vec<char> = Vec::with_capacity(chars.len());

        if col_size == 0 {
            return s;
        }

        // println!("{}", col_size);

        let mut current_middle_size = middle_size;
        let step = if middle_size == 0 { 1 } else { 2 };

        for r in 0..num_rows as usize {
            let mut col_index: usize = 0;
            // each rows.
            let start_value = r;

            while col_index < col_size {
                // col by col
                let mut char_index = start_value + step * col_index;

                if chars.get(char_index).is_none() {
                    col_index += 1;
                    break;
                } else {
                    result.push(*chars.get(char_index).unwrap());
                }

                // println!(
                //     "{},{},{},{}",
                //     col_index,
                //     char_index,
                //     start_value,
                //     chars.get(char_index).unwrap_or(&'-')
                // );

                col_index += 1;

                if current_middle_size == 0 && middle_size == 0 {
                    // no middle size, just step by step.
                    continue;
                }

                // num in middle.
                let mid_col_index = col_index + current_middle_size;
                char_index = start_value + step * mid_col_index;
                col_index += middle_size;
                if current_middle_size >= middle_size {
                    // skip.
                    continue;
                }
                if chars.get(char_index).is_some() {
                    result.push(*chars.get(char_index).unwrap());
                }
                // println!(
                //     "-- {},{},{},{}, {}",
                //     col_index,
                //     char_index,
                //     start_value,
                //     current_middle_size,
                //     chars.get(char_index).unwrap_or(&'-')
                // );
            }

            if current_middle_size <= 0 {
                current_middle_size = middle_size;
            } else {
                current_middle_size -= 1;
            }
        }

        result.into_iter().collect::<String>()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_get_column_size() {
        let colsize = Solution::get_column_size(4, 14);
        assert_eq!(colsize, 7);
    }
}
