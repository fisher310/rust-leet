use crate::common::*;

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => {
                let mut new_head = Self::reverse_list(node.next.take());
                if new_head.is_some() {
                    let mut tail = new_head.as_mut();
                    while tail.as_mut().unwrap().next.is_some() {
                        tail = tail.unwrap().next.as_mut();
                    }
                    tail.as_mut().unwrap().next = Some(node);
                    new_head
                } else {
                    Some(node)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let ans = Solution::reverse_list(create_list(&[1, 2, 3, 4, 5]));
        println!("{}", ans.unwrap());
    }
}
