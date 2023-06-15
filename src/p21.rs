use crate::common::*;
struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let (mut list1, mut list2) = (list1, list2);
        let mut current = &mut head;
        loop {
            match (list1, list2) {
                (Some(mut a), Some(mut b)) => {
                    if a.val < b.val {
                        list1 = a.next.take();
                        list2 = Some(b);
                        current = &mut current.insert(a).next;
                    } else {
                        list1 = Some(a);
                        list2 = b.next.take();
                        current = &mut current.insert(b).next;
                    }
                }
                (x, y) => {
                    *current = x.or(y);
                    break;
                }
            }
        }

        head
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
