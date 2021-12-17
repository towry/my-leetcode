/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
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
fn append_num(origin: i32, next: i32) -> i32 {
    origin * 10 + next
}

fn num_of_list(list: &Option<Box<ListNode>>) -> i32 {
    let mut val = 0;
    let mut node: &Option<Box<ListNode>> = list;

    while let Some(next) = &node.as_ref().unwrap().next {
        val = append_num(val, next.val);

        match node {
            None => (),
            Some(n) => node = &n.next,
        }
    }

    val
}

fn num_to_vec_in_revert(n: i32) -> Vec<i32> {
     n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .rev()
        .collect()
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l1n = num_of_list(&l1);
        let l2n = num_of_list(&l2);

        let result = l1n + l2n;
        let list = num_to_vec_in_revert(result);
        
        let mut root: Box<ListNode> = Box::new(ListNode::new(list[0]));
        let mut node: &mut Option<Box<ListNode>> = &mut root.next;

        let mut new_node;

        for n in list.iter().skip(1) {
            new_node = Some(Box::new(ListNode::new(*n)));
            match node {
                Some(ref mut n) => n.next = new_node,
                None => (),
            }
            node = &mut new_node;
        }

        Some(root)
    }
}
// @lc code=end

