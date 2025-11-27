use std::thread;

pub fn execute_spawm() {
    let handle = thread::spawn(|| {
        println!("hello from a thread");
        42
    });

    let result = handle.join().expect("thread panicked");
    println!("thread returned {}", result);
}
