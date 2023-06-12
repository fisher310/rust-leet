use crate::common::*;
struct Solution;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let l = Self::min_depth(n.deref().borrow().left.clone());
                let r = Self::min_depth(n.deref().borrow().right.clone());
                if l == 0 {
                    r + 1
                } else if r == 0 {
                    l + 1
                } else {
                    l.min(r) + 1
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        // [3,9,20,null,null,15,7]
        let res = Solution::min_depth(create_tree(&[
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]));
        assert_eq!(2, res);
    }

    #[test]
    fn test1() {
        // [2,null,3,null,4,null,5,null,6]
        let res = Solution::min_depth(create_tree(&[
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]));
        assert_eq!(5, res);
    }
}
