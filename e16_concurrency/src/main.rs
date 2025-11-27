use std::thread;

use e16_concurrency::{mpsc::execute_mpsc, shared_state::execute_shared_state, synchronized_threads::execute_synchronized_threads};

fn main() {
    println!("outside before thread 1");
    println!("outside before thread 2");
    println!("outside before thread 3");
    println!("outside before thread 4");

    let t: thread::JoinHandle<()> = thread::spawn(|| {
        println!("inside thread 1");
        println!("inside thread 2");
        println!("inside thread 3");
        println!("inside thread 4");
        println!("inside thread 5");
        println!("inside thread 6");
        println!("inside thread 7");
    });

    // added to allow thread reach completion
    let _ = t.join();

    println!("outside thread after 1");
    println!("outside thread after 2");
    println!("outside thread after 3");
    println!("outside thread after 4");

    // execute_mpsc();
    // execute_shared_state();
    execute_synchronized_threads();
}
