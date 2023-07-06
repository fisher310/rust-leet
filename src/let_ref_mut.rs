#[derive(Debug)]
struct Items(u32);

#[test]
fn test_main() {
    let items = Items(2);
    let items_ptr = &items;
    let ref items_ref = items;

    assert_eq!(items_ptr as *const Items, items_ref as *const Items);

    let mut a = Items(20);

    // {
        let ref mut b = a;
        b.0 += 25;
    // }

    println!("{:?}", items);
    println!("{:?}", a);


    let a = "abc";
    let a_ptr = &a;
    let a_clone = a.clone();

    assert_eq!(a_ptr as *const &str, a_ptr as *const &str);
    assert_ne!(a_ptr as *const &str, &a_clone as *const &str);
}
