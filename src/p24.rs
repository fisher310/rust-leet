use crate::common::*;

struct Solution;

// [1,2,3,4] -> [2,1,4,3]
// [] -> []
// [1] -> [1]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => match node.next {
                Some(mut child) => {
                    let p = child.next;
                    node.next = Self::swap_pairs(p);
                    child.next = Some(node);

                    Some(child)
                }
                None => Some(node),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ans = Solution::swap_pairs(create_list(&[1, 2, 3, 4]));
        println!("{:?}", ans);
    }

    #[test]
    fn test1() {
        let ans = Solution::swap_pairs(create_list(&[]));
        println!("{:?}", ans);
    }

    #[test]
    fn test2() {
        let ans = Solution::swap_pairs(create_list(&[1]));
        println!("{:?}", ans);
    }

    #[test]
    fn test3() {
        let ans = Solution::swap_pairs(create_list(&[1, 2, 3]));
        println!("{:?}", ans);
    }
}
