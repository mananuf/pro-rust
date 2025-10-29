use std::{collections::HashMap, mem::size_of_val, process::Output};
use std::ops::Mul;

type UserProfile = (String, u32, bool);

#[derive(Debug, Clone, Copy)]
struct  Milliseconds(u64);

#[derive(Debug, Clone, Copy)]
struct  Bytes(u64);

impl Mul for Bytes {
    type Output = Bytes;
    fn mul(self, rhs: Bytes) -> Bytes {
        Bytes(self.0 * rhs.0)
    }
}

impl Mul for Milliseconds {
    type Output = Milliseconds;
    fn mul(self, rhs: Milliseconds) -> Milliseconds {
        Milliseconds(self.0 * rhs.0)
    }
}

fn main() {
// Create a program that declares variables of type:
// i8, u32, f64, char, bool, and &str
// Print their sizes using std::mem::size_of_val()

let ph_level: i8 = 14;
println!("{}", size_of_val(&ph_level)); // 1 byte

let is_awake: bool = false;
println!("{}", size_of_val(&is_awake)); // 1 byte

let ph_level2: i16 = 14;
println!("{}", size_of_val(&ph_level2)); // 2 byte

let mut class_number: u32 = 500;
println!("{}", size_of_val(&class_number)); // 4byte

let grade: char = 'A';
println!("{}", size_of_val(&grade)); // 4byte

let points: f64 = 100.0;
println!("{}", size_of_val(&points)); // 8 byte

let ph_level3: u64 = 14;
println!("{}", size_of_val(&ph_level3)); // 8 byte

let fullname: &str = "Bankat Mananaf";
println!("{}", size_of_val(fullname)); // 1byte per string literal

let students: Vec<&str> = vec!["Bankat Mananaf"];
println!("{}", size_of_val(&students)); // 24 byte

let numbers: Vec<u8> = Vec::new();
println!("{}", size_of_val(&numbers)); // 24 byte

let numbers2: [u64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
println!("{}", size_of_val(&numbers2)); // 80 byte

let numbers2: [u64; 1] = [1];
println!("{}", size_of_val(&numbers2)); // 8 byte for each u64 integer

let numbers2: [u8; 1] = [1];
println!("{}", size_of_val(&numbers2)); // 1 byte for each u8 integer

let numbers2: [String; 1] = [String::from("H")];
println!("STRING {}", size_of_val(&numbers2)); // 24 byte for each string

let strings_: String = String::from("Hello World!");
println!("{}", size_of_val(&strings_)); // 24 byte

let strings_2: String = String::from("H");
println!("{}", size_of_val(&strings_2)); // 24 byte

let items: HashMap<String, i8> = HashMap::new();
println!("{}", size_of_val(&items));

// Experiment with ownership:
// Move a String into a function and observe compiler errors.
// Fix it using clone() and later using references &str.

move_string(strings_);
take_mutable_ref_of_int(&mut class_number);

// Tuple Accessor:
// Create a tuple (String, u32, bool) called user_profile.
// Write a function that destructures it and returns a summary string.

let user_profile: UserProfile = (String::from("Bnk Mnf"), 32, false);
println!("{}",destructure_tuple(user_profile));
}

fn move_string(s: String) -> String {
    s
}

fn take_mutable_ref_of_int(i: &mut u32) -> &mut u32 {
    *i = *i * 2;
    i
}

fn destructure_tuple(t: UserProfile) -> String {
    let (name, age, status) = t;

    format!("Name: {name}\nAge: {age}\nStatus: {status}")
}

fn transfer(size: Bytes) -> Bytes{
    size * Bytes(2)
}

fn wait(ms: Milliseconds) -> Milliseconds{
    ms * Milliseconds(2)
}
