struct Person(String);

#[test]
fn test_main() {
    let a = Person("Richcar Feynman".to_string());

    match a {
        Person(ref name) => println!("{} was a great physicist !", name),
        _ => panic!("Oh no !"),
    }

    let b = a;
}
