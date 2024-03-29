use std::cell::Cell;

#[derive(Debug, Clone)]
struct Bag {
    item: Box<u32>,
}

#[test]
fn test_main() {
    let bag = Cell::new(Bag { item: Box::new(1) });

    let hand1 = &bag;
    let hand2 = &bag;

    hand1.set(Bag { item: Box::new(2) });
    hand2.set(Bag { item: Box::new(3) });
}
