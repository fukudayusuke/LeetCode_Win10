/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
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
use std::mem;

impl Solution {
     pub fn merge_two_lists(mut l1: Option<Box<ListNode>>,mut l2: Option<Box<ListNode>>,) -> Option<Box<ListNode>> {
          if l1.is_none() {
               return l2;
          }
          if l2.is_none() {
               return l1;
          }

          let node1 = l1.as_mut().unwrap();
          let node2 = l2.as_mut().unwrap();

          if node1.val < node2.val {
               node1.next = Solution::merge_two_lists(mem::replace(&mut node1.next, None), l2);
               l1
          } else {
               node2.next = Solution::merge_two_lists(l1, mem::replace(&mut node2.next, None));
               l2
          }
     }
}
// @lc code=end

