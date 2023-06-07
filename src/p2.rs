use crate::common::*;
struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        help(l1, l2, 0)
    }
}

fn help(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut rem: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && rem == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: help(
                l1.and_then(|x| {
                    rem += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    rem += x.val;
                    x.next
                }),
                rem / 10,
            ),
            val: { rem % 10 },
        }))
    }
}

mod tests {

    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let l1 = common::create_list(vec![2,4,3].as_slice());
        let l2 = common::create_list(vec![5,6,4].as_slice());
        let res = Solution::add_two_numbers(l1, l2);
        println!("{:?}", res)
    }
}
