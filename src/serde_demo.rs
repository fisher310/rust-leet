use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Foo {
    name: String,
    age: u64,
}

impl Foo {
    fn new(name: &str, age: u64) -> Self {
        Foo {
            name: name.to_string(),
            age,
        }
    }
}

#[test]
fn test_main() {
    let foo_json = serde_json::to_string(&Foo::new("Fisher", 40)).unwrap();
    println!("{:?}", foo_json);

    let foo_value: Foo = serde_json::from_str(&foo_json).unwrap();
    println!("{:?}", foo_value);
}
