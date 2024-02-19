use std::{fs::File, io::Read, path::Path};

#[test]
#[should_panic]
fn test_main() {
    let path = Path::new("main.rs");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Error while opening file: {}", err),
    };

    let mut s = String::new();

    let res = file.read_to_string(&mut s);
    println!("res: {:?}", res);
    println!("message: {}", s);
}
