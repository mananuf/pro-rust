use std::{sync::{Arc, Barrier, Mutex}, thread::{self, JoinHandle}};

pub fn execute_synchronized_threads() {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let tasks: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let barrier: Arc<Barrier> = Arc::new(Barrier::new(11));

    for i in 0..=10 {
        let cloned_tasks: Arc<Mutex<Vec<String>>> = Arc::clone(&tasks);
        let barrier  = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            cloned_tasks.lock().unwrap().push(format!("thread {i} for task 1"));

            // used to wait for first task set to complete
            barrier.wait();

            cloned_tasks.lock().unwrap().push(format!("thread {i} for task 2"));
        });

        handles.push(handle);
    } 

    handles.into_iter().for_each(|handle| handle.join().unwrap());

    let tasks_lock = tasks.lock().unwrap();
    tasks_lock.iter().for_each(|task| println!("{task}"));
}