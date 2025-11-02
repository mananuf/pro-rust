use e06_structs::{Book, BookBuilder, Person, User};

fn main() {
    let p = Person {
    name: String::from("Mananaf"),
    age: 25,
};

// unpacking Structs
let Person {ref name, age} = p;
println!("{}, {age}", name);
let new_name = name;
println!("{new_name}");
println!("{name}");

// let new_person = p;
// println!("{p:#?}");

let things_fall_apart = Book::new("things fall apart".into(), true).build();

println!("{things_fall_apart:#?}");

let samankwe_and_the_highway_robbers = Book::new(
    "samankwe_and_the_highway_robbers".into(), 
    true,
).pages(144)
.build();

println!("{samankwe_and_the_highway_robbers:?}");

// TASK2:  Create a new user u1, then create u2 using struct update syntax based on u1.
let u1 = User { name: "mananaf".into(), age: 23};
let u2 = User {name: "akie".into(), ..u1};

println!("{u1:?}, {u2:?}")

}
