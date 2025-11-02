# Bridging Functional Principles + OOP Design in a Systems Language

Rust’s type system is one of its strongest features. Structs sit at the center of expressing data shapes, invariants, and relationships. To master Rust, you must understand not just how to use structs — but why, when, and the trade-offs behind them.

## 1. Why Do We Need Structs in Rust?

Rust is:

Not OOP, but allows encapsulation.

Not purely functional, but embraces immutability and transformations.

A systems language, so it must express memory layout explicitly.

### **Structs solve a crucial need:**
grouping related data with predictable memory layout, while enforcing Rust’s ownership model.

Consider them:

- Rust's "record types"

- A safer replacement for C’s struct

- A flexible alternative to classes (without inheritance)

- Structs help model domain concepts cleanly:

        User, Order, Point, Config, etc.

- They make code expressive, readable, and maintainable.

## 2. When Do You Need Structs?

Use Structs when you need:

- A cohesive data model
```rust
struct User {
    id: u64,
    name: String,
}
```
- Clear ownership boundaries

        Structs help define who owns what.

- Strong typing for domain logic

        Rust avoids primitive obsession.

- Performance & memory layout control

        Structs are stored contiguously in memory → cache-friendly.

- Self-documenting APIs

    A function accepting `UserProfile` conveys intention better than multiple parameters.

## 3. Types of Structs in Rust

Rust has 3 kinds of structs:

## 3.1 Named Structs

Classic struct with named fields.
```rust
struct Person {
    name: String,
    age: u8,
}
```

Best for:

- Domain models

- Configurations

- Anything with multiple fields

## 3.2 Tuple Structs

Fields without names — just like a tuple, but with a custom type.
```rust
struct Color(u8, u8, u8);
```

Use when:

- Field names do not add clarity

- You want a newtype that wraps a single item:
```rust
struct UserId(u64);
```

Very useful for type safety.

## 3.3 Unit Structs

No fields.
```rust
struct Marker;
```

Use cases:

- Zero-sized types (ZST)

- Compile-time markers

- Type-level programming

## 5. Ownership of Structs and Their Fields
A struct owns all its fields.
```rust
let user = User { name: String::from("Naf"), id: 10 };
```

MEMORY MODEL:
```cpp
    user (stack)
    │
    ├── id: u64 (stack)
    └── name: String (stack pointer → heap)
```