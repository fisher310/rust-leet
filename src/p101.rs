use crate::common::*;
struct Solution;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(x) => {
                let l = x.deref().borrow().left.clone();
                let r = x.deref().borrow().right.clone();
                symmetric_helper(l, r)
            }
        }
    }
}

fn symmetric_helper(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(pp), Some(qq)) => {
            if pp.deref().borrow().val == qq.deref().borrow().val {
                symmetric_helper(
                    pp.deref().borrow().left.clone(),
                    qq.deref().borrow().right.clone(),
                )
                &&
                symmetric_helper(
                    pp.deref().borrow().right.clone(), 
                    qq.deref().borrow().left.clone())
            } else {
                false
            }
        }
        (None, None) => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0 () {
        let res = Solution::is_symmetric(create_tree(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]));
        assert_eq!(false, res);
    }

    #[test]
    fn test1() {
        let res = Solution::is_symmetric(create_tree(&[Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)]));
        assert_eq!(true, res);
    }

    #[test]
    fn test2() {
        let res = Solution::is_symmetric(create_tree(&[Some(2), Some(3), Some(3), Some(4), Some(5), None, Some(4)]));
        assert_eq!(false, res);
    }
}