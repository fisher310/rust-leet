use std::{borrow::Borrow, collections::BinaryHeap, fmt::Binary, ops::Deref};

use crate::common::*;
struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });

        let mut heap = BinaryHeap::new();

        for n in lists {
            heap.push(n.unwrap().val);
        }

        dummy.next
    }
}
