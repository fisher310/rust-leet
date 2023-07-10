use std::{sync::mpsc::channel, thread, time::Duration};

#[test]
fn test_main() {
    let (tx, rx) = channel();
    let join_handle = thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("Received {}", n);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 0..10 {
        tx.send(i).unwrap();
    }
    drop(tx);

    join_handle.join().unwrap();
}
