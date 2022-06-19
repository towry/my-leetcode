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
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let middle_index = total_len / 2;
        let is_even = total_len % 2 == 0;
        let mut left_cursor_index: usize = 0;
        let mut right_cursor_index: usize = 0;
        let mut in_order_index: usize = 0;
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;
        while in_order_index < middle_index {
            let left1 = nums1.get(left_cursor_index);
            let left2 = nums1.get(left_cursor_index + 1);
            let right1 = nums2.get(right_cursor_index);
            let right2 = nums2.get(right_cursor_index + 1);

            if Some()
            if left_cursor_index < nums1.len() {
                num1 = nums1[left_cursor_index];
                if num1 <= num2 {
                    left_cursor_index += 1;
                    in_order_index += 1;
                }
            } else if right_cursor_index < nums2.len() {
                num1 = nums2[right_cursor_index];
                right_cursor_index += 1;
                in_order_index += 1;
            } else {
                num1 = 0;
            }
        }
        if is_even {
            return (num1 as f64 + num2 as f64) / 2.0;
        }
        return num2 as f64;
    }
}
// @lc code=end

