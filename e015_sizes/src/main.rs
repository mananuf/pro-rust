use std::{collections::HashMap, marker::PhantomData};

fn main() {
    println!("size of bool {}", size_of::<bool>()); // 1 byte
    println!("size of (i64, i64) {}", size_of::<(i64, i64)>()); // 16 byte


    println!("size of i8 {}", size_of::<i8>()); // 1 byte
    println!("size of i16 {}", size_of::<i16>()); // 2 byte
    println!("size of i32 {}", size_of::<i32>()); // 4 byte
    println!("size of i64 {}", size_of::<i64>()); // 8 byte

    println!("size of u8 {}", size_of::<u8>()); // 1 byte
    println!("size of u16 {}", size_of::<u16>()); // 2 byte
    println!("size of u32 {}", size_of::<u32>()); // 4 byte
    println!("size of u64 {}", size_of::<u64>()); // 8 byte

    println!("size of f32 {}", size_of::<f32>()); // 4 byte
    println!("size of f64 {}", size_of::<f64>()); // 8 byte

    println!("size of &[i8] {}", size_of::<&[i8]>()); // 16 byte
    println!("size of [i8; 1] {}", size_of::<[i8; 1]>()); // 1 byte
    println!("size of [i8; 10] {}", size_of::<[i8; 10]>()); // 10 byte

    println!("size of &[i64] {}", size_of::<&[i64]>()); // 16 byte
    println!("size of &str {}", size_of::<&str>()); // 16 byte
    println!("size of String {}", size_of::<String>()); // 24 byte
    println!("size of Box<String> {}", size_of::<Box<String>>()); // 8 byte
    println!("size of Box<i64> {}", size_of::<Box<i64>>()); // 16 byte

    println!("memory word size of pc {}", size_of::<()>()); // 16 byte

    struct A(String, i64);
    println!("size of struct A(String, i64) {}", size_of::<A>()); // 32 byte

    struct B {a: String, b: i64}
    println!("size of struct B [a: String, b: i64] {}", size_of::<B>()); // 32 byte
    println!("size of HashMap<i64, String> {}", size_of::<HashMap<i64, String>>()); // 48 byte
    println!("size of HashMap<STring, String> {}", size_of::<HashMap<String, String>>()); // 48 byte

    trait Shape {
        
    }

    println!("size of &dyn Shape {}", size_of::<&dyn Shape>()); // 48 byte

    let mut c: String = "hello".to_string();
    let e = &mut c;

    (*e).push_str("string");

    println!("{e}");

    println!("size of PhantomData<&dyn Shape> {}", size_of::<PhantomData<&dyn Shape>>()); // 0 byte
    // slice
    // let a: &[i64] = &[1,2]; 
    // print!("{a:?}")
}