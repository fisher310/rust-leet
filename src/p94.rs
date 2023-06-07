use crate::common::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = Vec::new();
        helper(root, &mut ret);
        ret
    }
}
fn helper(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
    if let Some(x) = node {
        let left: Option<Rc<RefCell<TreeNode>>> = x.deref().borrow().left.clone();
        helper(left, arr);
        arr.push(x.deref().borrow().val);
        let right = x.deref().borrow().right.clone();
        helper(right, arr);
    }
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let root = common::create_tree(&[Some(1), None, Some(2), Some(3)]);
        let ret = Solution::inorder_traversal(root);

        assert_eq!(vec![1, 3, 2], ret);
    }

    #[test]
    fn test2() {
        let root = common::create_tree(&[Some(1)]);
        let ret = Solution::inorder_traversal(root);
        assert_eq!(vec![1], ret);
    }

    #[test]
    fn test3() {
        let root = common::create_tree(&[]);
        let ret = Solution::inorder_traversal(root);
        assert_eq!(Vec::<i32>::new(), ret);
    }
}
