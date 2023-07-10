use std::{sync::mpsc, thread};

#[test]
fn test_main() {
    let (tx, rx) = mpsc::sync_channel(1);

    let tx_clone = tx.clone();

    let _ = tx.send(0);

    thread::spawn(move || {
        let _ = tx.send(1);
    });

    thread::spawn(move || {
        let _ = tx_clone.send(2);
    });

    println!("received {} via the channel", rx.recv().unwrap());
    println!("received {} via the channel", rx.recv().unwrap());
    println!("received {} via the channel", rx.recv().unwrap());
}
