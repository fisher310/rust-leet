use crate::common::*;
struct Solution;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(pp), Some(qq)) => {
                if pp.deref().borrow().val == qq.deref().borrow().val {
                    Self::is_same_tree(
                        pp.deref().borrow().left.clone(),
                        qq.deref().borrow().left.clone(),
                    ) && Self::is_same_tree(
                        pp.deref().borrow().right.clone(),
                        qq.deref().borrow().right.clone(),
                    )
                } else {
                    false
                }
            }
            (None, None) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;
    #[test]
    fn test0() {
        let p = common::create_tree(&[Some(1), Some(2), Some(3)]);
        let q = common::create_tree(&[Some(1), Some(2), Some(3)]);
        let res = Solution::is_same_tree(p, q);
        assert_eq!(true, res);
    }

    #[test]
    fn test1 () {
        let p = common::create_tree(&[Some(1), None, Some(3)]);
        let q = common::create_tree(&[Some(1), Some(2), Some(3)]);
        let res = Solution::is_same_tree(p, q);
        assert_eq!(false, res);
    }
}
