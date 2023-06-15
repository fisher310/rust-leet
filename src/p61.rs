use crate::common::*;

struct Solution;

// [1,2,3,4,5] -> [4,5,1,2,3]
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut head = head;
        let mut ptr = &head;

        let mut len = 0;
        while let Some(ref n) = ptr {
            len += 1;
            ptr = &n.next;
        }
        let k = k % len;
        if k == 0 {
            return head;
        }

        let mut ptr = &mut head;
        for _ in 1..len - k {
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        let mut new_head = ptr.as_mut().unwrap().next.take();
        let mut tail = &mut new_head;
        while tail.is_some() && tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }

        tail.as_mut().unwrap().next = head;

        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let ans = Solution::rotate_right(create_list(&[1, 2, 3, 4, 5]), 2);
        match ans {
            Some(list) => println!("{}", list),
            None => println!("None"),
        }
    }
}
