fn take_the_n(n: &mut u8) {
    *n += 2;
}

fn take_the_s(s: &mut String) {
    s.push_str("ing");
}

#[test]
fn test_name() {
    let mut n = 5;
    let mut s = String::from("Borrow");

    take_the_n(&mut n);
    take_the_s(&mut s);

    println!("n change to {}", n);
    println!("s change to {}", s);
}
