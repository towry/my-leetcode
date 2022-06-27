/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] Reverse Integer
 *
 * https://leetcode.cn/problems/reverse-integer/description/
 *
 * algorithms
 * Medium (35.32%)
 * Likes:    3543
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 2.9M
 * Testcase Example:  '123'
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If
 * reversing x causes the value to go outside the signed 32-bit integer range
 * [-2^31, 2^31 - 1], then return 0.
 *
 * Assume the environment does not allow you to store 64-bit integers (signed
 * or unsigned).
 *
 *
 * Example 1:
 *
 *
 * Input: x = 123
 * Output: 321
 *
 *
 * Example 2:
 *
 *
 * Input: x = -123
 * Output: -321
 *
 *
 * Example 3:
 *
 *
 * Input: x = 120
 * Output: 21
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_neg = x < 0;
        let mut rev_num_str = x.to_string().chars().collect::<Vec<_>>();

        if is_neg {
            rev_num_str.push('-');
        }

        rev_num_str.reverse();

        if is_neg {
            rev_num_str.pop();
        }

        rev_num_str
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .map_or(0, |v| v)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(-20), -2);
        assert_eq!(Solution::reverse(-222222222), -222222222);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-0), 0);
    }
}
