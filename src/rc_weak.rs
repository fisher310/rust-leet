use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<LinkedListNode<T>>>,
}

#[derive(Debug)]
struct LinkedListNode<T> {
    next: Option<Rc<LinkedListNode<T>>>,
    prev: RefCell<Option<Weak<LinkedListNode<T>>>>,
    data: T,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, data: T) -> Self {
        let new_node = Rc::new(LinkedListNode {
            data,
            next: self.head.clone(),
            prev: RefCell::new(None),
        });

        match self.head.clone() {
            Some(node) => {
                let mut prev = node.prev.borrow_mut();
                *prev = Some(Rc::downgrade(&new_node))
            }
            None => {}
        }

        LinkedList {
            head: Some(new_node),
        }
    }
}
