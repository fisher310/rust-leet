use std::{cell::RefCell, fmt::Display, rc::Rc};

use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut t = &self.next;

        let mut ret = String::new();
        ret.push_str(&format!("{} -> ", self.val));
        loop {
            match t {
                None => break,
                Some(ref x) => {
                    ret.push_str(&format!("{} -> ", x.val));
                    t = &x.next;
                }
            }
        }
        ret.push_str("null");
        f.write_fmt(format_args!("[{}]", &ret))
    }
}

pub fn create_list(list: &[i32]) -> Option<Box<ListNode>> {
    if list.is_empty() {
        None
    } else {
        Some(Box::new(ListNode {
            val: list[0],
            next: create_list(&list[1..]),
        }))
    }
}

fn print_list(list: Option<Box<ListNode>>) {
    let s = print_help(list);
    let s = s.trim_end_matches("\\s,");
    println!("[{}]", s);
}

fn print_help(list: Option<Box<ListNode>>) -> String {
    match list {
        None => "".to_owned(),
        Some(x) => format!("{}, {}", x.val, print_help(x.next)),
    }
}

///
/// tree implement
///

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn create_tree(arr: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None;
    }

    let mut queue = VecDeque::new();
    let root = match arr[0] {
        None => None,
        Some(x) => Some(Rc::new(RefCell::new(TreeNode::new(x)))),
    };

    queue.push_back(root.clone());
    let mut i = 1;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap().unwrap();
        if i < arr.len() {
            if let Some(x) = arr[i] {
                let left = Some(Rc::new(RefCell::new(TreeNode::new(x))));
                queue.push_back(left.clone());
                current.borrow_mut().left = left;
            }
            i += 1;
        }
        if i < arr.len() {
            if let Some(x) = arr[i] {
                let right = Some(Rc::new(RefCell::new(TreeNode::new(x))));
                queue.push_back(right.clone());
                current.borrow_mut().right = right;
            }
            i += 1;
        }
    }

    root
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let l1 = create_list(vec![1, 2, 3].as_slice());
        println!("{:?}", l1);
    }

    #[test]
    fn test_print_list() {
        let list = create_list(&[1, 2, 3, 4, 54]);
        print_list(list);
    }

    #[test]
    fn test_list_display() {
        let list = create_list(&[1, 2, 3, 5, 4, 2]);
        match list {
            None => println!("None"),
            Some(l) => println!("{}", l),
        }
    }

    #[test]
    fn test_create_tree() {
        let res = create_tree(&[Some(1), None, Some(2), Some(3)]);
        println!("{:?}", res);
    }
}
