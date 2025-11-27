use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};

struct File{
    text: Vec<String>
}
pub fn execute_shared_state() {
    let file: Arc<Mutex<File>> = Arc::new(Mutex::new(File { text: vec![]}));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for val in 1..=10 {
        let file = Arc::clone(&file);

        let handle =thread::spawn(move || {
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("value of thread {val}"));
        });

        handles.push(handle);
    }

    // Always join your threads to ensure they complete before using their results.
    // await all spwaned threads to finish computing
    handles.into_iter().for_each(|handle| handle.join().unwrap());
    // ↑ BLOCKS until ALL threads complete their work

    let file_lock = file.lock().unwrap();

    file_lock.text.iter().for_each(|text| println!("{text}"));
}