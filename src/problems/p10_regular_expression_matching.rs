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
#![allow(clippy::needless_return, dead_code)]

struct Solution;

// @lc code=start

/// Used to mark the pattern that currently uesd.
#[derive(Debug)]
enum MatchingPattern {
    /// No pattern left
    None,
    /// abc
    Chars(Vec<char>),
    /// single dot
    SingleAny,
    /// \[char\]\*
    MoreChar(char),
    /// .*
    MoreAny,
}

#[derive(Debug)]
pub struct MyRegex {
    /// The pattern string to be matched.
    pattern: Vec<char>,
    /// The string to be tested.
    strings: Vec<char>,
    /// The pattern splitted in group.
    matching_patterns: Vec<MatchingPattern>,
    /// mutable cursor.
    cursor: RegexCursor,
}

#[derive(Debug)]
struct RegexCursor {
    is_forward: bool,
    /// string index forward
    forward_sindex: usize,
    /// string index backward
    backward_sindex: usize,
    /// pattern index forward
    forward_pindex: usize,
    /// pattern index backward
    backward_pindex: usize,
    /// `.*`
    is_match_more_any: bool,
    /// `char*`
    is_match_more_char: bool,
}

impl RegexCursor {
    fn get_sindex(&self) -> usize {
        return if self.is_forward {
            self.forward_sindex
        } else {
            self.backward_sindex
        };
    }
    /// get range from string.
    fn sindex_range(&self, size: usize) -> Option<std::ops::Range<usize>> {
        return if self.is_forward {
            // 0 1 2 3
            let end = self.forward_sindex + size;
            if end > self.backward_sindex + 1 {
                return None;
            }
            Some(self.forward_sindex..end)
        } else {
            let end = self.backward_sindex - size + 1;
            if end < self.forward_sindex {
                return None;
            }
            Some(end..self.backward_sindex + 1)
        };
    }
    fn get_pindex(&self) -> usize {
        return if self.is_forward {
            self.forward_pindex
        } else {
            self.backward_pindex
        };
    }
    fn toggle_forward(&mut self) {
        self.is_forward = !self.is_forward;
    }
    fn inc_sindex(&mut self) {
        if self.is_forward {
            self.forward_sindex += 1;
        } else {
            self.backward_sindex -= 1;
        }
    }
    fn move_sindex(&mut self, size: usize) {
        if self.is_forward {
            self.forward_sindex += size;
        } else {
            self.backward_sindex -= size;
        }
    }
    fn inc_pindex(&mut self) {
        if self.is_forward {
            self.forward_pindex += 1;
        } else {
            self.backward_pindex -= 1;
        }
    }
}

impl MyRegex {
    fn new(pattern: Vec<char>, strings: Vec<char>) -> Self {
        let backward_sindex = strings.len() - 1;
        MyRegex {
            pattern,
            strings,
            matching_patterns: vec![],
            cursor: RegexCursor {
                is_forward: true,
                forward_sindex: 0,
                backward_sindex,
                forward_pindex: 0,
                backward_pindex: 0,
                is_match_more_any: false,
                is_match_more_char: false,
            },
        }
    }
}

impl MyRegex {
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
                    self.matching_patterns.push(MatchingPattern::MoreAny);
                } else if pre_char != ' ' {
                    group.pop();
                    if !group.is_empty() {
                        self.matching_patterns.push(MatchingPattern::Chars(group));
                        group = vec![];
                    }
                    self.matching_patterns
                        .push(MatchingPattern::MoreChar(pre_char));
                }
                continue;
            } else if *s == '.' {
                pre_char = '.';
                if !group.is_empty() {
                    self.matching_patterns.push(MatchingPattern::Chars(group));
                    group = vec![];
                }
                self.matching_patterns.push(MatchingPattern::SingleAny);
                continue;
            }

            pre_char = c.cloned().unwrap();
            group.push(pre_char);
        }

        if !group.is_empty() {
            self.matching_patterns.push(MatchingPattern::Chars(group));
        }
    }

    fn get_next_pattern_item(&self) -> Option<&MatchingPattern> {
        return self.matching_patterns.get(self.cursor.get_pindex());
    }

    fn is_match(&mut self) -> bool {
        self.before_match();

        let strings = &self.strings;

        loop {
            let pat = self.get_next_pattern_item();
            if pat.is_none() {
                break;
            }
            let pat = pat.unwrap();
            match pat {
                MatchingPattern::Chars(parts) => {
                    let parts_len = parts.len();
                    let Some(range) = self.cursor.sindex_range(parts_len) else {
                        return false;
                    };
                    let mirror = &strings[range];
                    if mirror == parts {
                        self.cursor.move_sindex(parts_len);
                        self.cursor.inc_pindex();
                        continue;
                    }
                    return false;
                }
                MatchingPattern::SingleAny => {
                    // dot
                    self.cursor.inc_sindex();
                    self.cursor.inc_pindex();
                }
                MatchingPattern::MoreChar(c) => {
                    let e = c.clone();
                    if self.cursor.is_match_more_char {
                        self.cursor.is_match_more_char = false;
                        self.cursor.toggle_forward();
                        // start match more char.
                        loop {
                            let Some(range) = self.cursor.sindex_range(1) else {
                                // match zero.
                                self.cursor.inc_pindex();
                                break;
                            };
                            let mirror = &strings[range];
                            if &mirror[0] == &e {
                                self.cursor.inc_sindex();
                            } else {
                                // match char ends.
                                self.cursor.inc_pindex();
                                break;
                            }
                        }
                        continue;
                    }
                    self.cursor.is_match_more_char = true;
                    self.cursor.toggle_forward();
                }
                MatchingPattern::MoreAny => {
                    if self.cursor.is_match_more_any {
                        // fish.
                        self.cursor.toggle_forward();
                        self.cursor.inc_pindex();
                        self.cursor
                            .move_sindex(self.cursor.backward_sindex - self.cursor.forward_sindex);
                        // start match more any.
                        continue;
                    }
                    self.cursor.toggle_forward();
                    self.cursor.is_match_more_any = true;
                }
                MatchingPattern::None => {}
            }
        }
        // b*ba => bba
        // (.*)(aaa)(b*)c(.*) => abaaaac

        return true;
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut reg = MyRegex::new(
            p.chars().collect::<Vec<char>>(),
            s.chars().collect::<Vec<char>>(),
        );
        reg.is_match()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_match_1() {
        assert!(!Solution::is_match("abc".to_owned(), "a*".to_owned()));
    }
    #[test]
    fn test_is_match_2() {
        assert!(Solution::is_match("abc".to_owned(), "a.*".to_owned()));
    }
    #[test]
    fn test_is_match_3() {
        assert!(Solution::is_match("abc".to_owned(), ".*".to_owned()));
    }
    #[test]
    fn test_is_match_4() {
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
    }
    #[test]
    fn test_is_match_5() {
        assert!(Solution::is_match("aa".to_owned(), "aa".to_owned()));
    }
    #[test]
    fn test_is_match_6() {
        assert!(Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
    }

    #[test]
    fn test_is_match_7() {
        assert!(!Solution::is_match("aaa".to_owned(), "aaaa".to_owned()));
    }

    #[test]
    fn test_is_match_8() {
        assert!(Solution::is_match("aaa".to_owned(), "a*a".to_owned()));
    }

    #[test]
    fn test_is_match_9() {
        assert!(Solution::is_match("aaa".to_owned(), "ab*ac*a".to_owned()));
    }

    #[test]
    fn test_is_match_10() {
        assert!(!Solution::is_match("ab".to_owned(), ".*c".to_owned()))
    }
}
