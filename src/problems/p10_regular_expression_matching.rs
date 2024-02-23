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
#[derive(Debug)]
pub enum Precedant {
    None,
    Char(char),
    DotChar,
}

#[derive(Debug)]
struct MyRegex {
    pattern: Vec<char>,
    pindex: u32,
    sindex: u32,
    precedant: Precedant,
}

const DOT_CHAR: char = '.';
const MANY_CHAR: char = '*';

impl MyRegex {
    fn is_special_char(&self, c: &char) -> bool {
        *c == DOT_CHAR || *c == MANY_CHAR
    }
    fn is_match(&mut self, s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();

        loop {
           let ch = self.pattern.get(self.pindex as usize);
           if ch.is_none() {
                break;
           }
           let p: &char = ch.unwrap();
           let sh = chars.get(self.sindex as usize);
           if sh.is_none() {
            if *p != MANY_CHAR {
                return false;
            }
            self.pindex = self.pindex + 1;
            if self.pattern.len() > self.pindex as usize {
                self.sindex = self.sindex - 1;
            }
            continue;
           }
           let s: &char = sh.unwrap();

           if !self.is_special_char(p) && *p == *s  {
            self.pindex = self.pindex + 1;
            self.sindex = self.sindex + 1;
            self.precedant = Precedant::Char(*p);
            continue;
           } else if !self.is_special_char(p) {
               if !matches!(self.precedant, Precedant::None) {
                    return false;
               }
               self.pindex = self.pindex + 1;
               self.precedant = Precedant::Char(*p);
           }

           if *p == DOT_CHAR {
            self.pindex = self.pindex +1;
            self.sindex = self.sindex + 1;
            self.precedant = Precedant::DotChar;
            continue;
           }

           if *p == MANY_CHAR {
            match self.precedant {
                Precedant::Char(ch) => {
                    // match 0 or multiple times
                    if ch == *s {
                        self.sindex = self.sindex + 1;
                    } else {
                        // no match, MANY_CHAR is invalid. 0 time.
                        self.pindex = self.pindex + 1;
                        self.precedant = Precedant::None;
                    }
                }
                Precedant::DotChar => {
                    self.sindex = self.sindex + 1;
                }
                _ => {
                    continue;
                }
            }
           }
        }
        // check char left
        if self.pattern.len() - self.pindex as usize > 1 {
            return false;
        }
        let ch = chars.get(self.sindex as usize);
        if ch.is_some() {
            return false;
        }

        true
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
        assert_eq!(
            Solution::is_match("abc".to_owned(), "a.*".to_owned()),
            true
        );
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
        assert_eq!(
            Solution::is_match("aaa".to_owned(), "a*a".to_owned()),
            true
        );
    }
    
    #[test]
    fn test_is_match_9() {
        assert_eq!(
            Solution::is_match("aaa".to_owned(), "ab*ac*a".to_owned()),
            true
        );
    }
}
