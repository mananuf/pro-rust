use std::{sync::{Arc, Mutex}, thread};

pub fn execute_scoped() {
    let mut v = vec![1,2,3];
    thread::scope(|s| {
        s.spawn(|| {
            // can't borrow v immutably/mutably here because of closure borrow rules,
            // but you *can* borrow short-lived references safely
            v.push(1);
        });
    });
    println!("{v:?}");

    let data = Arc::new(Mutex::new(vec![]));
{
    let mut v = data.lock().unwrap_or_else(|pe| pe.into_inner());
    v.push(1);
}
}
