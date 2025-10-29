fn main() {
    let a: u8 = 5;
    let b = &a;
    let z: char = 'x';

    let s: String = String::from("Hello");
    let m = &s; 

    // if you meant to write a `char` literal, use single quotes: `'y'
    // let y: char = "y"; // expected `char`, found `&str`

    println!("{:p} {:p}", &a, b); 
    println!("{}", *b);
    println!("{:p} {:p}", &s, m); 

    // Immutable variable (stack)
    let x = 42;
    println!("x = {x}");

    // Mutable variable
    let mut counter = 0;
    counter += 1;
    println!("counter = {counter}");

    // Shadowing
    let counter = counter * 10;
    println!("shadowed counter = {counter}");

    // Heap-allocated variable
    let name = String::from("Mananaf");
    println!("Hello, {name}");

    // ownership
    ownership_demo();

    // efficient data handling
    efficient_handling();

    let strings_array = vec![String::from("still"), String::from("1234567"), String::from("Mananaf"), String::from("Aisha")];
    let result = longest_string(&strings_array);
    println!("longest: {result}");
}

fn ownership_demo() {
    let s1 = String::from("Rust");
    let s2 = s1;             // s1 moved into s2
    // println!("{s1}");     // out of scope
    println!("{s2}");

    let s3 = String::from("Borrow");
    print_ref(&s3);          // borrow
    println!("{s3}");        // still valid
}

fn print_ref(value: &String) {
    println!("Borrowed: {value}");
}

fn efficient_handling() {
    // Prefer borrowing over cloning
    let title = String::from("Rust in Production");
    print_summary(&title);  // borrow
    println!("After borrow: {title}");

    // Use copy types for simple data
    let temp_c: f32 = 36.6;
    display_temp(temp_c);   // copied
}

fn print_summary(summary: &str) {
    println!("Summary: {summary}");
}

fn display_temp(temp: f32) {
    println!("Temp: {temp}");
}

fn longest_string(array_of_strings: &[String]) -> &String {
    array_of_strings.iter().max_by_key(|s| s.len()).unwrap()
}