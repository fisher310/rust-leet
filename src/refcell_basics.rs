use std::cell::RefCell;

#[derive(Debug)]
struct Bag {
    item: Box<u32>,
}

#[test]
fn test_main() {
    let bag = RefCell::new(Bag { item: Box::new(32) });

    let hand1 = &bag;
    let hand2 = &bag;

    *hand1.borrow_mut() = Bag { item: Box::new(2) };
    *hand2.borrow_mut() = Bag { item: Box::new(3) };
    *hand1.borrow_mut() = Bag { item: Box::new(4) };
    *hand2.borrow_mut() = Bag { item: Box::new(5) };

    {
        let borrowed = hand1.borrow();
        println!("{:?}", borrowed);
    }
    *hand1.borrow_mut() = Bag { item: Box::new(4) };
}
