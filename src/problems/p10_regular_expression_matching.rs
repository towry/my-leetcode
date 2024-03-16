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
    sindex: usize,
    is_match_many: bool,
    matching_patterns: Vec<MatchingPattern>,
}

impl MyRegex {
    fn new(pattern: Vec<char>) -> Self {
        MyRegex {
            pattern,
            sindex: 0,
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
) {
    let parts_len = parts.len();
    let sindex_init = *sindex;

    loop {
        let mirror = &chars[*sindex..=parts_len];
        if mirror == parts {
            *sindex += 1;
        } else if *sindex != sindex_init {
            *sindex = *sindex + parts_len;
            // done
            break;
        } else if *is_in_matching_any {
            // start matching any
            *sindex = *sindex + parts_len;
            *is_in_matching_any = false;
            break;
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

        loop {
            let pat = self.matching_patterns.get(i);
            if pat.is_none() {
                break;
            }
            let pat = pat.unwrap();
            match pat {
                MatchingPattern::Chars(parts) => {
                    matching_chars(&chars, parts, &mut self.sindex, &mut self.is_match_many);
                }
                MatchingPattern::Any => {
                    // dot*
                }
                MatchingPattern::ManyRepeat => {
                    self.is_match_many = true;
                }
                _ => {}
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
