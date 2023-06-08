use crate::common::*;
struct Solution;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let l = Self::max_depth(n.deref().borrow().left.clone());
                let r = Self::max_depth(n.deref().borrow().right.clone());
                l.max(r) + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let r = create_tree(&[Some(1), None, Some(2), Some(3)]);
        let res = Solution::max_depth(r);
        assert_eq!(3, res);
    }
}
