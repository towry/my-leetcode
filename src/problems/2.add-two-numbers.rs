/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.cn/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (41.85%)
 * Likes:    8201
 * Dislikes: 0
 * Total Accepted:    1.4M
 * Total Submissions: 3.3M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
struct ArithValue {
    reminder: i32,
    value: i32
}

impl Solution {
    fn get_reminder_and_value(l: &Option<Box<ListNode>>, r: &Option<Box<ListNode>>, pre_reminder: i32) -> ArithValue {
        let ln = l.as_ref().map_or(0, |node| node.val);
        let rn = r.as_ref().map_or(0, |node| node.val);

        let total = (ln + rn + pre_reminder);
        let value = total % 10;
        let reminder = if total >= 10 { 1 } else { 0 };
        println!("{}, {}, {}, {}, {}", total, value, ln, rn, pre_reminder);
        ArithValue {
            reminder: reminder,
            value: value
        }
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut left_node = l1;
        let mut right_node = l2;
        let mut av = Solution::get_reminder_and_value(&left_node, &right_node, 0);
        let mut root = Box::new(ListNode::new(av.value));

        let mut root_ref = &mut root;

        while left_node.is_some() || right_node.is_some() {
            left_node = if left_node.is_some() { left_node.unwrap().next } else { None };
            right_node = if right_node.is_some() { right_node.unwrap().next } else { None };
            if left_node.is_none() && right_node.is_none() && av.reminder == 0 {
                break;
            }
            av = Solution::get_reminder_and_value(&left_node, &right_node, av.reminder);
    
            root_ref.next = Some(Box::new(ListNode::new(av.value)));
            root_ref = root_ref.next.as_mut().unwrap();
        
        } 

        Some(root)
    }
}
// @lc code=end

