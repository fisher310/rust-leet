use crate::common::*;
struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut ll1 = Box::new(ListNode {
            val: 0,
            next: list1,
        });
        let mut ll2 = Box::new(ListNode {
            val: 0,
            next: list2,
        });
        let mut current = &mut dummy as *mut Box<ListNode>;
        let mut ll1 = &mut ll1 as *mut Box<ListNode>;
        let mut ll2 = &mut ll2 as *mut Box<ListNode>;
        unsafe {
            loop {
                match (&(*ll1).next, &(*ll2).next) {
                    (Some(x), Some(y)) => {
                        if x.val < y.val {
                            // (*current).next = Some(Box::new(ListNode::new(x.val)));
                            // ll1 = (*ll1).next.as_mut().unwrap();
                            (*current).next = (*ll1).next.take();
                            ll1 = (*current).next.as_mut().unwrap();
                        } else {
                            // (*current).next = Some(Box::new(ListNode::new(y.val)));
                            // ll2 = (*ll2).next.as_mut().unwrap();
                            (*current).next = (*ll2).next.take();
                            ll2 = (*current).next.as_mut().unwrap();
                        }
                        current = (*current).next.as_mut().unwrap();
                    }
                    (Some(x), None) => {
                        // (*current).next = Some(Box::new(ListNode::new(x.val)));
                        // ll1 = (*ll1).next.as_mut().unwrap();
                        (*current).next = (*ll1).next.take();
                        ll1 = (*current).next.as_mut().unwrap();
                        current = (*current).next.as_mut().unwrap();
                    }
                    (None, Some(x)) => {
                        // (*current).next = Some(Box::new(ListNode::new(x.val)));
                        // ll2 = (*ll2).next.as_mut().unwrap();
                        (*current).next = (*ll2).next.take();
                        ll2 = (*current).next.as_mut().unwrap();
                        current = (*current).next.as_mut().unwrap();
                    }
                    (None, None) => {
                        break;
                    }
                }
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let ans = Solution::merge_two_lists(create_list(&[1, 3, 5]), create_list(&[2, 4]));
        println!("{:?}", ans);
    }

    #[test]
    fn test1() {
        let ans = Solution::merge_two_lists(create_list(&[]), create_list(&[]));
        println!("{:?}", ans);
    }
    #[test]
    fn test2() {
        let ans = Solution::merge_two_lists(create_list(&[]), create_list(&[1]));
        println!("{:?}", ans);
    }
}
