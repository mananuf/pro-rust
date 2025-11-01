// Practice identifying whether a type implements Copy.
// All primitive scalars (i32, bool, char, etc.) + tuples of Copy types do.
// Rust is smart: a borrow ends when last used, not strictly at the end of scope.
// Deref happens implicitly in many cases
// Box<T>, Rc<T>, Arc<T>, custom smart pointers → all use Deref trait.

// ✅ Professional Hint:
// Always define string arguments as &str in public APIs.
// This makes them accept both String and &str.

use std::collections::HashMap;

fn main() {
    let int_test: i32 = 24;
    let boolean_test: bool = false;
    let characters: char = 'D';
    let floats: f32 = 1.2345;
    let response: (bool, &str) = (true, "success");

    let _copy_int = int_test;
    let _copy_boolean = boolean_test;
    let _copy_characters = characters;
    let _copy_floats = floats;
    let _copy_response = response;


    println!("{int_test} {boolean_test} {characters} {floats} {response:?}");
    // primitive data types implements Copy by default.

    let name: String = String::from("Hello man!");
    let students: Vec<String> = vec!["hyper".to_string(), "motion".to_string()];
    let users: HashMap<String, u64> = HashMap::new();
    let b = Box::new(5);
    let c = b;

    println!("{}", *c);

    let _copy_names = name;
    let _copy_students = students;
    let _copy_users = users;

    // print!("{name}"); // borrow of moved value: `name`
    // print!("{students:?}") // borrow of moved value: `students`
    // print!("{users:?}") // borrow of moved value: `users`
    // print!("{b}"); // borrow of moved value: `b`

    // compound/Heap/Reference Data types does not implement the Copy Trait, so by default their values are moved

    // Compound Data Types vs Collections ?

    let mut nums = vec![1,2,3,4];
    nums.iter_mut().for_each(|n| *n += 10);

    println!("{nums:?}");
    // for n in &nums {
    //     nums.push(n + 10);
    // }
}

// consider using the `'static` lifetime, but this is uncommon 
// unless you're returning a borrowed value from a `const` or a `static`: `'static `
// missing lifetime specifier this function's return type contains a borrowed value, 
// but there is no value for it to be borrowed from
// fn return_ref_data() -> &String{
//     &"good morning!".to_string()
// } 

// FIX: add lifetime specifier
// cos the problem occurs as a result of the owned data we're referencing, goes out of scope before the reference is returned.
// so automatically, the return data will return a dangling pointer, for data that has beeen dropped. 
// that is why the function didn't compile

// returns a reference to data owned by the current function
fn return_ref_data_fix<'a>(input: &'a String) -> &'a String {
    input
}

fn return_back_ownership_of_owned_string() -> String{
    "good morning!".to_string()
} 
