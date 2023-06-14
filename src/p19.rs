use crate::common::*;
struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        let mut fast = &mut dummy as *mut Box<ListNode>;
        let mut slow = &mut dummy as *mut Box<ListNode>;
        unsafe {

            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }

            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_raw_point() {
        let mut v = vec![1_i64, 2_i64, 3_i64];

        let p = v.as_mut_ptr();
        unsafe {
            println!("{:?}, {:?}", p, p.add(1));
            assert_eq!(*p, 1);
            let p = p.add(1);
            *p = 100_i64;
            let p = p.add(1);
            *p = 900_i64;
            println!("{:?}", p);
        }

        println!("{:?}", v);
    }

    #[test]
    fn test0 () {
        let head = create_list(&vec![1,2,3,4,5]);
        let head = Solution::remove_nth_from_end(head, 2);
        println!("{:?}", head);
    }
}
