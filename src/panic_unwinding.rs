use std::thread;

fn alice() -> thread::JoinHandle<()> {
    thread::spawn(
        move || {
            bob();
        },
    )
}

fn bob() {
    malice();
}

fn malice() {
    panic!("malice is panicking!");
}

#[test]
fn test_main() {
    let child = alice();
    let _ = child.join();
    bob();
    println!("This is unreachable code.");
}
