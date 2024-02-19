use std::{
    sync::{Arc, Mutex},
    thread,
};

#[test]
fn test_main() {
    let nums = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for n in 0..5 {
        let ns = nums.clone();
        let t = thread::spawn(move || {
            let mut ns = ns.lock().unwrap();
            ns.push(n);
        });

        childs.push(t);
    }

    for c in childs {
        c.join().unwrap();
    }

    println!("nums: {:?}", nums.lock().unwrap());
}
