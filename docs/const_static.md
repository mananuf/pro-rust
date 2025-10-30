
## 1. The Big Picture

Rust manages memory primarily in three regions:

* **Stack:** for local, short-lived values (fast access)
* **Heap:** for dynamically allocated values (via `Box`, `Vec`, etc.)
* **Data segment:** for constants and statics (read-only or global)

Now, let’s break down how `const`, `static`, and `&str` fit into this.

---

## 2. `const` — **Compile-time Constant (Copied into Each Use Site)**

`const` in Rust:

* Is **inlined** wherever it is used (like a macro constant).
* Has **no fixed memory address** — each use creates its own copy.
* Is **immutable** and **evaluated at compile time**.
* Stored in the **read-only data segment** if its value is not inlined immediately.

### Example:

```rust
const MAX_CONNECTIONS: u32 = 100;

fn main() {
    println!("Max connections: {}", MAX_CONNECTIONS);
    let x = MAX_CONNECTIONS + 5;
}
```

**Key point:**
Each usage of `MAX_CONNECTIONS` embeds the value `100` directly into the generated code.
There’s **no single memory address** holding it.

**Use case:**
Use `const` for **compile-time constants** — things like configuration limits, mathematical constants, or conversions known at compile time.

---

## 3. `static` — **Global Variable with a Fixed Memory Address**

A `static` in Rust:

* Has **one memory location** for the entire program.
* Lives for the **entire program duration** (global lifetime).
* Can be **mutable**, but mutation requires `unsafe` access.
* Stored in the **data segment** (initialized before `main()`).

### Example:

```rust
static MAX_THREADS: u32 = 8;
static mut COUNTER: u32 = 0;

fn main() {
    println!("Max threads: {}", MAX_THREADS);
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
```

**Key point:**
Unlike `const`, a `static` **has one address** that all parts of the program refer to.
You can even get a pointer to it:

```rust
println!("{:p}", &MAX_THREADS);
```

**Safety note:**
`static mut` is `unsafe` because concurrent access can cause data races.
Instead, prefer using **atomic types** or **Mutex** for thread-safe statics.

**Use case:**
Use `static` for **global configuration**, **caches**, **singletons**, or **static lookup tables** that must persist.

---

## 4. `&'static str` — **String Slice with Static Lifetime**

A string literal like `"Hello, Rust!"` is actually:

* A **reference** to a string stored in **read-only memory**.
* Type: `&'static str`.

### Example:

```rust
fn main() {
    let greeting: &'static str = "Hello, Rust!";
    println!("{}", greeting);
}
```

**Key point:**
The string `"Hello, Rust!"` is **baked into the binary** and lives for the entire program lifetime.
So `"Hello, Rust!"` → **stored in the read-only data segment**, and
`greeting` → is a pointer to it.

**Use case:**
Use `&'static str` when working with **string literals** or **static program messages** that don’t need dynamic allocation.

---

## 🏗️ 5. Summary Table

| Feature                | `const`                   | `static`                  | `&'static str`         |
| ---------------------- | ------------------------- | ------------------------- | ---------------------- |
| Lifetime               | None (copied inline)      | `'static` (global)        | `'static` (global)     |
| Memory location        | Inlined or read-only data | Data segment              | Read-only data segment |
| Mutability             | Immutable                 | Mutable only via `unsafe` | Immutable              |
| Addressable            | ❌ No                      | ✅ Yes                     | ✅ Yes                  |
| Evaluated at           | Compile time              | Runtime (once)            | Compile time           |
| Use case               | Compile-time constants    | Global/shared state       | String literals        |
| Thread-safe by default | ✅                         | ❌ (if `mut`)              | ✅                      |

---

## 6. Visual Analogy

Imagine a restaurant kitchen:

* `const` → Each chef (function) has their **own copy** of the recipe.
* `static` → The restaurant has **one master recipe** pinned to the wall.
* `&'static str` → The **menu text** printed on the wall — same, unchangeable, and available to everyone forever.

---

## ⚡ Example Comparing All

```rust
const PI: f64 = 3.14159;
static GREETING: &str = "Hello from static!";
static mut COUNTER: u32 = 0;

fn main() {
    let message: &'static str = "Hello from literal!";

    println!("Const PI: {}", PI);
    println!("Static str: {}", GREETING);
    println!("Literal: {}", message);

    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
```

**Memory placement:**

* `PI` → compile-time inlined constant (no address)
* `GREETING` → pointer in **data segment**, string literal in **read-only segment**
* `message` → pointer to **read-only segment**
* `COUNTER` → mutable global in **data segment**

---