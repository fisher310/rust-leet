use std::{sync::Arc, thread, time::Duration};

#[test]
fn test_main() {
    let nums = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut childs = vec![];
    for n in 0..5 {
        let ns = Arc::clone(&nums);
        let c = thread::spawn(move || {
            println!("{}", ns[n]);
            thread::sleep(Duration::from_millis(100));
        });

        childs.push(c);
    }

    for c in childs {
        c.join().unwrap();
    }
}
