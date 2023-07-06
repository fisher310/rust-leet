
fn get_a_borrowed_value<'a>() -> Box<u8> {
    let x = 1;
    Box::new(x)
}

#[test]
fn test_name () {
    let value = get_a_borrowed_value();

    
}