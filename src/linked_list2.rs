use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Box<Node<T>>>>>;

struct Node<T> {
    data: Rc<T>,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: Rc::new(data),
            prev: None,
            next: None,
        }
    }

    fn set_prev(&mut self, other: &Link<T>) {
        self.prev = other.clone();
    }

    fn set_next(&mut self, other: &Link<T>) {
        self.next = other.clone();
    }

    fn get_prev(&self) -> Link<T> {
        self.prev.clone()
    }

    fn get_next(&self) -> Link<T> {
        self.next.clone()
    }

    fn get_data(&self) -> Rc<T> {
        self.data.clone()
    }

    fn new_link(data: T) -> Link<T> {
        Some(Rc::new(RefCell::new(Box::new(Node::new(data)))))
    }
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push(&mut self, data: T) {
        let new_node = Node::new_link(data);
        self.len += 1;
        if self.head.is_none() && self.tail.is_none() {
            self.head = new_node.clone();
            self.tail = new_node;
        } else {
            self.tail.as_ref().unwrap().borrow_mut().set_next(&new_node);
            new_node.as_ref().unwrap().borrow_mut().set_prev(&self.tail);
            self.tail = new_node;
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Node::new_link(data);
        self.len += 1;
        if self.head.is_none() && self.tail.is_none() {
            self.head = new_node.clone();
            self.tail = new_node;
        } else {
            new_node.as_ref().unwrap().borrow_mut().set_next(&self.head);
            self.head.as_ref().unwrap().borrow_mut().set_prev(&new_node);
            self.head = new_node;
        }
    }

    pub fn pop(&mut self) -> Option<Rc<T>> {
        if self.head.is_none() && self.tail.is_none() {
            return None;
        }

        let old_tail = self.tail.clone();
        self.tail = old_tail.as_ref().unwrap().borrow().get_prev();
        self.tail.as_ref().unwrap().borrow_mut().set_next(&None);
        let old_data = old_tail.as_ref().unwrap().borrow().get_data();
        Some(old_data)
    }

    pub fn pop_front(&mut self) -> Option<Rc<T>> {
        if self.head.is_none() && self.tail.is_none() {
            return None;
        }

        let old_headd = self.head.clone();
        self.head = old_headd.as_ref().unwrap().borrow().get_next();
        self.head.as_ref().unwrap().borrow_mut().set_prev(&None);
        let old_data = old_headd.as_ref().unwrap().borrow().get_data();
        self.len -= 1;
        Some(old_data)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_push_back_length() {
        let mut new_list = LinkedList::<i32>::new();
        let values = (0..10).collect::<Vec<i32>>();
        for i in values {
            new_list.push(i);
        }
        assert_eq!(new_list.len(), 10);
    }

    #[test]
    fn test_push_front_length() {
        let mut new_list = LinkedList::<i32>::new();
        let values = (0..10).collect::<Vec<i32>>();
        for i in values {
            new_list.push_front(i)
        }
        assert_eq!(new_list.len(), 10);
    }
}
