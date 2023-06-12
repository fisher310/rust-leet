use crate::common::*;
struct Solution;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        balance_helper(root).1
    }
}

fn balance_helper(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    match node {
        None => (0, true),
        Some(n) => {
            let (dl, bl) = balance_helper(n.deref().borrow().left.clone());
            let (dr, br) = balance_helper(n.deref().borrow().right.clone());
            (dl.max(dr) + 1, bl && br && (dl - dr).abs() <= 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let res = Solution::is_balanced(create_tree(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]));
        assert_eq!(false, res);
    }

    #[test]
    fn test1 () {
        let res = Solution::is_balanced(create_tree(&[]));
        assert_eq!(true, res);
    }

    #[test]
    fn test2 () {
        let res = Solution::is_balanced(create_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]));
        assert_eq!(true, res);
    }
}
