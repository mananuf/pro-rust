use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

enum List {
    Node(i32, Option<Box<List>>),
    Nil,
}

fn main() {
    let list = List::Node(32, Some(Box::new(List::Node(1, None))));

    let data: Rc<String> = Rc::new(String::from("data here"));
    println!("count: {}", Rc::strong_count(&data));

    let a: Rc<String> = Rc::clone(&data);
    println!("count: {}", Rc::strong_count(&data));

    let b: Rc<String> = Rc::clone(&data);
    println!("count: {}", Rc::strong_count(&data));

    let c: Rc<String> = Rc::clone(&data);
    println!("count: {}", Rc::strong_count(&data));

    let d: Rc<String> = Rc::clone(&data);
    println!("count: {}", Rc::strong_count(&data));

    println!("{a}\n{b}\n{c}\n{d}");

    // ARC

    for _ in 0..5 {
        let data: Arc<String> = Arc::new(String::from("New data"));
        // todo: fix
        // println!("arc count: {}", Arc::strong_count(&data));

        thread::spawn(move || {
            println!("{}", data);
        });
    }

    // Cell
    let mut info: Cell<String> = Cell::new("Initial text".to_string());
    println!("BEFORE {}", info.get_mut());
    info.set("new value".to_string());
    println!("AFTER {}", info.get_mut());

    // RefCell
    let data: Rc<RefCell<String>> = Rc::new(RefCell::new(String::from("data here")));
    let a: Rc<RefCell<String>> = Rc::clone(&data);
    let b: Rc<RefCell<String>> = Rc::clone(&data);
    let c: Rc<RefCell<String>> = Rc::clone(&data);

    a.borrow_mut().push_str(" A edited");
    b.borrow_mut().push_str(" B edited");
    c.borrow_mut().push_str(" C edited");

    println!("{:?} {:?}", &data, a);

    let arc_data = Arc::new(Mutex::new(String::from("data here")));

    for val in 0..5 {
        let arc_data_clone = arc_data.clone();

        thread::spawn(move || {
            let mut lock = arc_data_clone.lock().unwrap();
            // lock.push_str(&format!("{val}"));
            *lock = format!("{val}")
        });
    }

    println!("ARC MUTEX: {:?}", *arc_data);

    // WEAK<T>
    // let parent = Rc::new(RefCell::new(1));
    // let child = Rc::new(RefCell::new(0));

    // child.borrow_mut().parent = Rc::downgrade(&parent); // Weak

}
