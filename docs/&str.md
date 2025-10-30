## 🔤 What is `&str`?

In Rust, `&str` is a **string slice** — a **view** into a sequence of UTF-8 bytes representing text.

Formally:

```rust
&str = (&u8 pointer, length)
```

It doesn’t own the data — it’s just a **borrowed reference** to a portion of a string.

---

## How `&str` Works Internally

Every `&str` is:

* A **reference to a sequence of bytes** (`u8`)
* Accompanied by a **length**
* **Immutable**
* **UTF-8 validated** (so always valid text)

For example:

```rust
let s: &str = "Hello";
```

Under the hood:

```
s = (pointer_to_data, length = 5)
```

The data `"Hello"` itself is stored **somewhere in memory**, and `s` just points to it.

---

## 1. Types of `&str`

There are two main categories:

### a. String Literal (`&'static str`)

When you write:

```rust
let s = "Hello, Rust!";
```

* The literal `"Hello, Rust!"` is embedded **directly into the program’s read-only memory segment**.
* The type is automatically **`&'static str`**.
* It **lives for the entire program duration**.

 Example:

```rust
fn main() {
    let greeting: &'static str = "Good morning, Jos!";
    println!("{}", greeting);
}
```

Memory view:

```
┌────────────────────────────┐
│ "Good morning, Jos!"       │ ← read-only data segment (.rodata)
└────────────────────────────┘
          ↑
      &str pointer
```

 **Key point:**
All string literals are **`&'static str`** — immutable and available throughout the program.

---

### b. Borrowed String Slice (`&str` from `String`)

When you slice a `String`, you get a **`&str`** that points to the string’s heap data.

Example:

```rust
fn main() {
    let name = String::from("Mananaf Bankat");
    let slice: &str = &name[0..7]; // "Mananaf"
    println!("{}", slice);
}
```

Here:

* The actual text `"Mananaf Bankat"` lives on the **heap** (because it’s a `String`).
* The `&str` slice (`slice`) lives on the **stack**, pointing to a portion of that heap memory.

Memory layout:

```
Stack:
  name  ─────┐
              │
Heap:         ▼
  "Mananaf Bankat"
  ^------^
   slice: &str = "Mananaf"
```

 **Key point:**
The `&str` is **only valid as long as** the original `String` (`name`) is alive.
If `name` is dropped, `slice` becomes invalid.

---

## 2. Comparison — `&str` vs `String`

| Feature           | `&str`                                   | `String`                           |
| ----------------- | ---------------------------------------- | ---------------------------------- |
| Ownership         | Borrowed (no ownership)                  | Owns data (heap allocated)         |
| Mutability        | Immutable                                | Mutable (via `.push_str()`)        |
| Storage location  | Stack (pointer + len)                    | Heap (actual bytes)                |
| Lifetime          | Depends on owner (`String` or `'static`) | `'static` or dynamic               |
| Example use       | Function arguments, literals             | Data building, modification        |
| Memory efficiency | Very lightweight (no heap)               | Heap allocation (slower to create) |

**Best practice:**

* Use `&str` when you **just need to read or reference text**.
* Use `String` when you **need to own or modify** text.

---

## 3. Practical Example — Function Parameters

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Mananaf"); // &'static str
    let user = String::from("Bankat");
    greet(&user);     // &String -> &str (auto coerced)
}
```

**Why `&str` in functions?**
It allows the function to accept **both** string literals and borrowed `String`s, making it **more flexible and efficient**.

---

## 4. Memory Locations Summary

| Example code                   | Type           | Memory location                     | Lifetime      |
| ------------------------------ | -------------- | ----------------------------------- | ------------- |
| `let a = "Jos";`               | `&'static str` | Read-only segment                   | `'static`     |
| `let b = String::from("Jos");` | `String`       | Heap (bytes), Stack (ptr, len, cap) | Until dropped |
| `let c = &b[..];`              | `&str`         | Stack (ptr + len), refers to heap   | Same as `b`   |

---

## 5. A Quick Visual Summary

```
String literal ("Hello")    --> stored in read-only memory (.rodata)
&'static str                 --> points directly to that literal
String::from("Hello")       --> heap-allocated copy
&str (slice from String)    --> points to heap data of String
```

---

## 6. When to Use `&str`

Use `&str` when:

* You want to **pass text to a function** without taking ownership.
* You are working with **string literals**.
* You are slicing part of an existing string.
* You want **lightweight, zero-allocation references**.

🚫 Avoid `&str` when:

* You need to **store**, **append**, or **modify** the string (use `String` instead).
* You need to **own** the text (e.g., return it from a function).

---

## Example — Mixed Usage

```rust
fn describe(city: &str) {
    println!("City: {}", city);
}

fn main() {
    let literal = "Jos"; // &'static str
    let owned = String::from("Bukuru"); // String
    let borrowed = &owned[..]; // &str

    describe(literal);
    describe(borrowed);
}
```

Works for both literal and owned strings.
That’s why **function parameters** in idiomatic Rust prefer `&str` over `String`.

---