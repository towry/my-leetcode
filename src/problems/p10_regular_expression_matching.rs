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

struct Solution;

// @lc code=start

/// Used to mark the pattern that currently uesd.
#[derive(Debug)]
enum MatchingPattern {
    /// No pattern left
    None,
    Chars(Vec<char>),
    /// Match any char
    Any,
    ManyChar(char),
    ManyRepeat,
}

const DOT_CHAR: char = '.';
const MANY_CHAR: char = '*';

#[derive(Debug)]
struct MyRegex {
    pattern: Vec<char>,
    pindex: usize,
    sindex: usize,
    matching_patterns: Vec<MatchingPattern>,
}

impl MyRegex {
    fn new(pattern: Vec<char>) -> Self {
        MyRegex {
            pattern,
            pindex: 0,
            sindex: 0,
            matching_patterns: vec![],
        }
    }
}

impl MyRegex {
    fn is_special_char(&self, c: &char) -> bool {
        *c == DOT_CHAR || *c == MANY_CHAR
    }

    fn before_match(&mut self) {
        let mut i = 0;
        let mut group: Vec<char> = vec![];
        let mut pre_char: char = ' ';

        loop {
            let c = self.pattern.get(i);
            i += 1;
            if c.is_none() {
                break;
            }

            let s = c.unwrap();

            if *s == '*' {
                if pre_char == '.' {
                    self.matching_patterns.pop();
                    self.matching_patterns.push(MatchingPattern::ManyRepeat);
                } else if pre_char != ' ' {
                    group.pop();
                    if group.len() > 0 {
                        self.matching_patterns.push(MatchingPattern::Chars(group));
                        group = vec![];
                    }
                    self.matching_patterns
                        .push(MatchingPattern::ManyChar(pre_char));
                }
                continue;
            } else if *s == '.' {
                pre_char = '.';
                if group.len() > 0 {
                    self.matching_patterns.push(MatchingPattern::Chars(group));
                    group = vec![];
                }
                self.matching_patterns.push(MatchingPattern::Any);
                continue;
            }

            pre_char = c.cloned().unwrap();
            group.push(pre_char);
        }

        if group.len() > 0 {
            self.matching_patterns.push(MatchingPattern::Chars(group));
        }
    }

    fn match_on_dot(&self) {}
    fn match_on_many(&self) {}

    fn peek_next_pairs_pattern_char(&self) -> (Option<char>, Option<char>) {
        (
            self.pattern.get(self.pindex).cloned(),
            self.pattern.get(self.pindex + 1).cloned(),
        )
    }

    fn next_matching_pattern(&self) -> MatchingPattern {
        match self.peek_next_pairs_pattern_char() {
            (Some(a), Some(b)) => {
                if !self.is_special_char(&a) && !self.is_special_char(&b) {
                    return MatchingPattern::Char(a);
                } else if a == DOT_CHAR && b == MANY_CHAR {
                    return MatchingPattern::ManyRepeat;
                } else if b == MANY_CHAR {
                    return MatchingPattern::ManyChar(a);
                }
                return MatchingPattern::None;
            }
            (Some(a), None) => {
                if a == MANY_CHAR {
                    return MatchingPattern::ManyRepeat;
                } else if a == DOT_CHAR {
                    return MatchingPattern::Any;
                }
                return MatchingPattern::Char(a);
            }
            _ => {
                return MatchingPattern::None;
            }
        }
    }

    fn increase_pattern_index_by_matching(&mut self, mp: MatchingPattern) {
        match mp {
            MatchingPattern::Char(_) | MatchingPattern::Any => {
                self.pindex += 1;
            }
            MatchingPattern::ManyChar(_) | MatchingPattern::ManyRepeat => {
                self.pindex += 2;
            }
            _ => {}
        }
    }

    fn is_match(&mut self, s: String) -> bool {
        self.before_match();

        let chars = s.chars().collect::<Vec<char>>();

        loop {
            let mp = self.next_matching_pattern();
            let s = chars.get(self.sindex).cloned();

            if s.is_none() {
                match mp {
                    MatchingPattern::ManyChar(_) | MatchingPattern::ManyRepeat => {
                        self.increase_pattern_index_by_matching(mp);
                    }
                    _ => {}
                }
                break;
            }

            let s = s.unwrap();
            let mut is_in_matching_repeat = false;
            let mut matching_repeat_index: usize = 0;
            let mut is_in_matching_many_char = false;
            let mut matching_many_char_index: usize = 0;

            match mp {
                MatchingPattern::Any => {
                    self.sindex += 1;
                }
                MatchingPattern::Char(c) => {
                    if c == s {
                        self.sindex += 1;
                        self.increase_pattern_index_by_matching(mp);
                    } else if is_in_matching_repeat {
                        self.sindex += 1;
                        // matching repeat to match multiple.
                        self.pindex = matching_repeat_index;
                        self.sindex = chars.len() - 1;
                        self.increase_pattern_index_by_matching(MatchingPattern::ManyRepeat);
                    } else {
                        return false;
                    }
                }
                MatchingPattern::ManyChar(c) => {
                    if c == s {
                        self.sindex += 1;
                    } else {
                        self.increase_pattern_index_by_matching(mp);
                    }
                }
                MatchingPattern::ManyRepeat => {
                    matching_repeat_index = self.pindex;
                    is_in_matching_repeat = true;
                    self.increase_pattern_index_by_matching(mp);
                }
                _ => {
                    // no pattern left
                    break;
                }
            }
        }
        // b*ba => bba
        // (.*)(aaa)(b*)c(.*) => abaaaac

        return false;
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut reg = MyRegex::new(p.chars().collect::<Vec<char>>());
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
