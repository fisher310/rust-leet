use crate::common::*;
struct Solution;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => target_sum == 0,
            Some(node) => {
                let left = node.deref().borrow().left.clone();
                let right = node.deref().borrow().right.clone();
                let val = node.deref().borrow().val;
                if left == None && right == None {
                    // leaf
                    target_sum == val
                } else if left == None {
                    Self::has_path_sum(right, target_sum - val)
                } else if right == None {
                    Self::has_path_sum(left, target_sum - val)
                } else {
                    Self::has_path_sum(left, target_sum - val)
                        || Self::has_path_sum(right, target_sum - val)
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
        // [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
        let root = create_tree(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        let ret = Solution::has_path_sum(root, 22);
        assert_eq!(true, ret);
    }

    #[test]
    fn test1() {
        let root = create_tree(&[Some(1), Some(2), Some(3)]);
        let ret = Solution::has_path_sum(root, 5);
        assert_eq!(false, ret);
    }
}
