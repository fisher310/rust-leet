use crate::common::*;
struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ptr = head.as_mut();

        while ptr.is_some() && ptr.as_ref().unwrap().next.is_some() {
            if ptr.as_ref().unwrap().val == ptr.as_ref().unwrap().next.as_ref().unwrap().val {
                let next = ptr.as_mut().unwrap().next.take();
                ptr.as_mut().unwrap().next = next;
            } else {
                ptr = ptr.unwrap().next.as_mut();
            }
        }

        head
    }
}
