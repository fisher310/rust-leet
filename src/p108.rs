use crate::common::*;
use crate::p4::help;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        helper(&nums)
    }
}

fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;
    Some(Rc::new(RefCell::new(TreeNode {
        val: nums[mid],
        left: helper(&nums[0..mid]),
        right: helper(&nums[mid + 1..]),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test0() {
        let res = Solution::sorted_array_to_bst(vec![1, 3]);
        println!("{:?}", res)
    }

    #[test]
    fn test1() {
        let res = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
        println!("{:?}", res);
    }
}
