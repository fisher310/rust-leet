use std::panic;

#[test]
fn test_main() {
    panic::catch_unwind(|| {
        panic!("Packing....");
    })
    .ok();

    println!("Survived that panic");
}

// 社区failure库