/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] Regular Expression Matching
 *
 * https://leetcode.cn/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (31.67%)
 * Likes:    3050
 * Dislikes: 0
 * Total Accepted:    288.9K
 * Total Submissions: 912.1K
 * Testcase Example:  '"aa"\n"a"'
 *
 * Given an input string s and a pattern p, implement regular expression
 * matching with support for '.' and '*' where:
 *
 *
 * '.' Matches any single character.​​​​
 * '*' Matches zero or more of the preceding element.
 *
 *
 * The matching should cover the entire input string (not partial).
 *
 *
 * Example 1:
 *
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'.
 * Therefore, by repeating 'a' once, it becomes "aa".
 *
 *
 * Example 3:
 *
 *
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 20
 * 1 <= p.length <= 30
 * s contains only lowercase English letters.
 * p contains only lowercase English letters, '.', and '*'.
 * It is guaranteed for each appearance of the character '*', there will be a
 * previous valid character to match.
 *
 *
 */

struct Solution;

// @lc code=start
pub enum PrecedantType {
    None,
    ALL_CHAR,
}

impl Solution {
    fn is_char_equal<'a>(a: &'a char, p: &'a char) -> bool {
        a == p || *p == '.'
    }

    pub fn is_match(s: String, p: String) -> bool {
        let ss: std::str::Chars = s.chars();
        let pp = p.chars().collect::<Vec<_>>();
        let mut p_index: usize = 0;
        let mut precedant: Option<char> = None;
        let mut precedant_type = PrecedantType::None;

        for s in ss {
            let mut found = false;

            while p_index < pp.len() || !found {
                let pc = pp.get(p_index).unwrap();
                if *pc == '.' {
                    precedant_type = PrecedantType::ALL_CHAR;
                    precedant = None;
                }
                if Solution::is_char_equal(&s, pc) {
                    p_index += 1;
                    precedant = None;
                    found = true;
                } else if *pc == '*' {
                    if precedant.is_some()
                        && Solution::is_char_equal(&s, precedant.as_ref().unwrap())
                    {
                        found = true;
                    }
                    precedant = None;
                    continue;
                } else if precedant.is_none() {
                    p_index += 1;
                } else if precedant.is_some() {
                    break;
                }
            }

            if !found {
                p_index = 0;
            }
        }

        if precedant.is_some() {
            return false;
        }

        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("abc".to_owned(), "a*".to_owned()), false);
        assert_eq!(Solution::is_match("abc".to_owned(), "a.*".to_owned()), true);
        assert_eq!(Solution::is_match("abc".to_owned(), ".*".to_owned()), true);
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::is_match("aa".to_owned(), "aa".to_owned()), true);
        assert_eq!(
            Solution::is_match("aab".to_owned(), "c*a*b".to_owned()),
            true
        );
    }
}
