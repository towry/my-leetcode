/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 *
 * https://leetcode.cn/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (41.44%)
 * Likes:    5529
 * Dislikes: 0
 * Total Accepted:    733.7K
 * Total Submissions: 1.8M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return
 * the median of the two sorted arrays.
 *
 * The overall run time complexity should be O(log (m+n)).
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 *
 *
 * Constraints:
 *
 *
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -10^6 <= nums1[i], nums2[i] <= 10^6
 *
 *
 */

// @lc code=start
use std::cmp::{Ord, Ordering, PartialEq};

struct Opt<'a>(Option<&'a i32>);

#[derive(PartialEq)]
enum OptOrdering {
    LessOrEqual,
    Greater,
    Nope,
}

impl<'a, 'b: 'a> Opt<'a> {
    fn cmp(&self, other: &'b Self) -> OptOrdering {
        if self.0.is_none() {
            return OptOrdering::Nope;
        }
        let ord = self.0.cmp(&other.0);
        if ord == Ordering::Less || other.0.is_none() {
            return OptOrdering::LessOrEqual;
        } else if ord == Ordering::Greater {
            return OptOrdering::Greater;
        } else {
            return OptOrdering::LessOrEqual;
        }
    }
}
// 1,2,3,4,5,6,7,8
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let is_even = total_len % 2 == 0;
        let middle_index = if is_even {
            total_len / 2 - 1
        } else {
            total_len / 2
        };
        let mut left_cursor_index: usize = 0;
        let mut right_cursor_index: usize = 0;
        let mut in_order_index: usize = 0;
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;
        if total_len <= 0 {
            return 0f64;
        }

        while in_order_index <= middle_index {
            let left1 = Opt(nums1.get(left_cursor_index));
            let left2 = Opt(nums1.get(left_cursor_index + 1));
            let right1 = Opt(nums2.get(right_cursor_index));
            let right2 = Opt(nums2.get(right_cursor_index + 1));

            if left1.cmp(&right1) == OptOrdering::LessOrEqual {
                num1 = *left1.0.unwrap();
                left_cursor_index += 1;
                in_order_index += 1;
                if left2.cmp(&right1) == OptOrdering::LessOrEqual {
                    num2 = *left2.0.unwrap();
                } else if right1.0.is_some() {
                    num2 = *right1.0.unwrap();
                }
            } else if right1.cmp(&left1) == OptOrdering::LessOrEqual {
                num1 = *right1.0.unwrap();
                right_cursor_index += 1;
                in_order_index += 1;
                // println!("{}, {}", in_order_index, middle_index);
                if right2.cmp(&left1) == OptOrdering::LessOrEqual {
                    num2 = *right2.0.unwrap();
                } else if left1.0.is_some() {
                    num2 = *left1.0.unwrap();
                }
            }
        }

        // println!("{}, {}", num1, num2);

        if is_even {
            return (num1 as f64 + num2 as f64) / 2.0;
        }
        return num1 as f64;
    }
}
// @lc code=end
