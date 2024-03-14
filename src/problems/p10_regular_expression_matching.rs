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
#![allow(clippy::needless_return)]

use std::cmp::{Eq, PartialEq};

struct Solution;

// @lc code=start
#[derive(Debug)]
pub enum Precedant {
    None,
    Char(char),
    DotChar,
}

#[derive(Debug)]
struct MyRegex {
    pattern: Vec<char>,
    pindex: usize,
    sindex: usize,
    precedant: Precedant,
}

struct MatchingGroup<'a>(&'a str);

impl<'a> PartialEq<&str> for MatchingGroup<'a> {
    fn eq(&self, rhs: &&str) -> bool {
        return self.0 == *rhs;
    }
}

const DOT_CHAR: char = '.';
const MANY_CHAR: char = '*';

impl MyRegex {
    fn is_special_char(&self, c: &char) -> bool {
        *c == DOT_CHAR || *c == MANY_CHAR
    }

    fn match_on_dot(&self) {}
    fn match_on_many(&self) {}

    fn match_on_char(&mut self, s: &char, p: &char) -> bool {
        if !self.is_special_char(p) && *p == *s {
            self.pindex += 1;
            self.sindex += 1;
            self.precedant = Precedant::None;
            return false;
        } else if !self.is_special_char(p) {
            self.pindex += 1;
            self.precedant = Precedant::Char(*p);
            return false;
        }

        if *p == DOT_CHAR {
            if matches!(self.precedant, Precedant::Char(_)) {
                return true;
            }
            self.pindex += 1;
            self.sindex += 1;
            self.precedant = Precedant::DotChar;
            return false;
        }

        if *p == MANY_CHAR {
            match self.precedant {
                Precedant::Char(ch) => {
                    // match 0 or multiple times
                    if ch == *s {
                        self.sindex += 1;
                    } else {
                        // no match, MANY_CHAR is invalid. 0 time.
                        self.pindex += 1;
                        self.precedant = Precedant::None;
                    }
                }
                Precedant::DotChar => {
                    self.sindex += 1;
                }
                _ => {
                    self.pindex += 1;
                }
            }
        }
        return false;
    }

    fn is_match(&mut self, s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();

        loop {
            let ch = self.pattern.get(self.pindex).cloned();
            let sh = chars.get(self.sindex);

            if ch.is_none() {
                if sh.is_none() {
                    return true;
                }
                return false;
            }
            let p: char = ch.unwrap();
            if sh.is_none() {
                break;
            }
            let s: &char = sh.unwrap();

            if self.match_on_char(s, &p) {
                return false;
            }
        }

        let sh = chars.get(self.sindex - 1);
        if sh.is_none() {
            return true;
        }
        let s = sh.unwrap();
        return false;
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut reg = MyRegex {
            pattern: p.chars().collect::<Vec<char>>(),
            pindex: 0,
            sindex: 0,
            precedant: Precedant::None,
        };

        reg.is_match(s)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_match_1() {
        assert_eq!(Solution::is_match("abc".to_owned(), "a*".to_owned()), false);
    }
    #[test]
    fn test_is_match_2() {
        assert_eq!(Solution::is_match("abc".to_owned(), "a.*".to_owned()), true);
    }
    #[test]
    fn test_is_match_3() {
        assert_eq!(Solution::is_match("abc".to_owned(), ".*".to_owned()), true);
    }
    #[test]
    fn test_is_match_4() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
    }
    #[test]
    fn test_is_match_5() {
        assert_eq!(Solution::is_match("aa".to_owned(), "aa".to_owned()), true);
    }
    #[test]
    fn test_is_match_6() {
        assert_eq!(
            Solution::is_match("aab".to_owned(), "c*a*b".to_owned()),
            true
        );
    }

    #[test]
    fn test_is_match_7() {
        assert_eq!(
            Solution::is_match("aaa".to_owned(), "aaaa".to_owned()),
            false
        );
    }

    #[test]
    fn test_is_match_8() {
        assert_eq!(Solution::is_match("aaa".to_owned(), "a*a".to_owned()), true);
    }

    #[test]
    fn test_is_match_9() {
        assert_eq!(
            Solution::is_match("aaa".to_owned(), "ab*ac*a".to_owned()),
            true
        );
    }
}
