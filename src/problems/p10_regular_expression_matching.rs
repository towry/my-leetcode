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
    Any,
    /// \[char\]\*
    ManyChar(char),
    /// .*
    ManyRepeat,
}

#[derive(Debug)]
pub struct MyRegex {
    pattern: Vec<char>,
    sindex: usize,
    /// .*
    is_match_many: bool,
    /// \[char\]\*
    is_match_many_char: bool,
    matching_patterns: Vec<MatchingPattern>,
}

impl MyRegex {
    fn new(pattern: Vec<char>) -> Self {
        MyRegex {
            pattern,
            sindex: 0,
            is_match_many_char: false,
            is_match_many: false,
            matching_patterns: vec![],
        }
    }
}

// use free function because of interprocedure conflict problem.
fn matching_chars(
    chars: &Vec<char>,
    parts: &Vec<char>,
    sindex: &mut usize,
    is_in_matching_any: &mut bool,
    is_in_matching_many_char: &mut bool,
) -> bool {
    let parts_len = parts.len();
    let sindex_init = *sindex;

    loop {
        if chars.len() < (*sindex + parts_len) {
            return false;
        }
        let mirror = &chars[*sindex..(*sindex + parts_len)];
        if *is_in_matching_any {
            if mirror == parts {
                *sindex += 1;
            } else {
                // TODO: fixme
                // should support backend match.
                // aba|c|a, .*|c|b*|a
                *sindex += parts_len;
            }
        } else if *is_in_matching_many_char {
            // (char)*
            if mirror == parts {
                *sindex += 1;
            } else if sindex_init != *sindex {
                *sindex += parts_len - 1;
                *is_in_matching_many_char = false;
                break;
            } else {
                // *abb, abb
                // check the match one case
                return false;
            }
        } else if mirror == parts {
            *sindex += parts_len;
            break;
        } else {
            return false;
        }
    }
    return true;
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
                    self.matching_patterns.push(MatchingPattern::ManyRepeat);
                } else if pre_char != ' ' {
                    group.pop();
                    if !group.is_empty() {
                        self.matching_patterns.push(MatchingPattern::Chars(group));
                        group = vec![];
                    }
                    self.matching_patterns
                        .push(MatchingPattern::ManyChar(pre_char));
                }
                continue;
            } else if *s == '.' {
                pre_char = '.';
                if !group.is_empty() {
                    self.matching_patterns.push(MatchingPattern::Chars(group));
                    group = vec![];
                }
                self.matching_patterns.push(MatchingPattern::Any);
                continue;
            }

            pre_char = c.cloned().unwrap();
            group.push(pre_char);
        }

        if !group.is_empty() {
            self.matching_patterns.push(MatchingPattern::Chars(group));
        }
    }

    fn is_match(&mut self, s: String) -> bool {
        self.before_match();

        let chars = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut match_many_char_back = false;

        loop {
            let pat = self.matching_patterns.get(i);
            if pat.is_none() {
                break;
            }
            let pat = pat.unwrap();
            match pat {
                MatchingPattern::Chars(parts) => {
                    let has_match = matching_chars(
                        &chars,
                        parts,
                        &mut self.sindex,
                        &mut self.is_match_many,
                        &mut self.is_match_many_char,
                    );
                    if !has_match {
                        if self.sindex >= chars.len() {
                            return true;
                        }
                        if self.is_match_many_char {
                            i -= 1;
                            match_many_char_back = true;
                            continue;
                        } else {
                            return false;
                        }
                    }
                    i += 1;
                }
                MatchingPattern::Any => {
                    // dot
                    self.sindex += 1;
                    i += 1;
                }
                MatchingPattern::ManyChar(c) => {
                    let next = self.matching_patterns.get(i + 1);
                    if !match_many_char_back && matches!(next, Some(MatchingPattern::Chars(_))) {
                        // match 0 first then match one char.
                        self.is_match_many_char = true;
                        i += 1;
                    } else if chars.get(self.sindex) == Some(c) {
                        // match one char.
                        self.sindex += 1;
                    } else {
                        // match 0.
                        self.is_match_many_char = false;
                        match_many_char_back = false;
                        i += 1;
                    }
                }
                MatchingPattern::ManyRepeat => {
                    let next = self.matching_patterns.get(i + 1);
                    if next.is_some() {
                        self.is_match_many = true;
                        i += 1;
                    } else {
                        self.is_match_many = false;
                        i += 1;
                        self.sindex += chars.len();
                    }
                }
                MatchingPattern::None => {}
            }
        }

        if self.is_match_many_char {
            return false;
        }
        if self.is_match_many {
            return true;
        }
        if self.sindex < chars.len() {
            return false;
        }
        // b*ba => bba
        // (.*)(aaa)(b*)c(.*) => abaaaac

        return true;
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
