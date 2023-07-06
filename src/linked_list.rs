use std::rc::Rc;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&mut self, data: T) -> Self {
        LinkedList {
            head: Some(Rc::new(Node {
                data,
                next: self.head.clone(),
            })),
        }
    }
}

#[test]
fn test_main() {
    let mut list_of_nums = LinkedList::new();

    let mut a = list_of_nums.append(1);
    println!("nums a: {:?}", a);
    let b = a.append(2);
    println!("nums: {:?}", list_of_nums);
    println!("nums a: {:?}", a);
    println!("nums b: {:?}", b);

    let list_of_strs = LinkedList::new().append("foo").append("bar");
    println!("strs: {:?}", list_of_strs);
}
