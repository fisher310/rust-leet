use crate::common::*;
struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));

        let mut curr = dummy.as_mut();
        while curr.as_ref().unwrap().next.is_some()
            && curr.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
        {
            if curr.as_ref().unwrap().next.as_ref().unwrap().val
                == curr
                    .as_ref()
                    .unwrap()
                    .next
                    .as_ref()
                    .unwrap()
                    .next
                    .as_ref()
                    .unwrap()
                    .val
            {
                let x = curr.as_ref().unwrap().next.as_ref().unwrap().val;
                while curr.as_ref().unwrap().next.is_some()
                    && curr.as_ref().unwrap().next.as_ref().unwrap().val == x
                {
                    curr.as_mut().unwrap().next =
                        curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
                }
            } else {
                curr = curr.unwrap().next.as_mut();
            }
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let ans = Solution::delete_duplicates(create_list(&[1, 1, 1, 2, 3, 3, 4]));
        match ans {
            None => {
                println!("None");
            }
            Some(node) => {
                println!("{}", node);
            }
        }
    }
}
