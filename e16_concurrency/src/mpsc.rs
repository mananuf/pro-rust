use std::{sync::{Arc, mpsc}, thread};

pub fn execute_mpsc() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tx);

    for val in 0..10 {
        let tx_clone = Arc::clone(&tx);
        
        thread::spawn(move || {
            let _ = tx_clone.send(val);
        });
    }

    //drop the transmission to aloow program stop receiving
    drop(tx);

    for msg in rx {
        println!("received: {msg}");
    }
}